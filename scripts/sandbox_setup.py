#!/usr/bin/env python3
"""Try pantomim against a real Moodle without setting one up.

Sets up an SEB quiz on the public Moodle Sandbox (https://sandbox.moodledemo.net) and a
pantomim profile in /tmp where the sandbox's demo student is already logged in, then prints
the command that opens the quiz. Your own pantomim cookies stay untouched.

    python3 -I scripts/sandbox_setup.py [--quit-url URL] [--quit-password PW]

The sandbox wipes itself every hour on the hour; run this again after that. It takes a few
minutes because the sandbox answers slowly. Needs only Python's standard library.
"""

import argparse, html, http.cookiejar, re, shutil, sys, time, urllib.parse, urllib.request
from pathlib import Path

SITE, OUT = "https://sandbox.moodledemo.net", Path("/tmp/pantomim-sandbox")
QUESTIONS = [  # (question, answers with the correct one first; Moodle shuffles them)
    ("In the OSI model, which layer does HTTP work at?", ["Application (layer 7)", "Transport (layer 4)", "Network (layer 3)", "Data link (layer 2)"]),
    ("Which port does HTTPS use by default?", ["443", "80", "21", "8080"]),
    ("Which device works at the network layer (layer 3)?", ["Router", "Hub", "Switch", "Repeater"]),
    ("Which transport protocol delivers data in order and without loss?", ["TCP", "UDP", "ICMP", "ARP"]),
]


def fail(message):
    sys.exit(f"\nFailed: {message}\nThe sandbox may be resetting (it does so on the hour). Try again in a few minutes.")


def attr(tag, name):
    found = re.search(rf'\s{name}="([^"]*)"', tag)
    return html.unescape(found.group(1)) if found else None


def form(page, action):
    """Where the first form posting to `action` goes, and the fields a browser would send."""
    found = re.search(rf'<form[^>]*action="([^"]*{action}[^"]*)"[^>]*>(.*?)</form>', page, re.S) or fail(f"no {action} form")
    body, fields = found.group(2), {}
    for tag, options in re.findall(r"(<select\b[^>]*>)(.*?)</select>", body, re.S):
        options = re.findall(r"<option\b[^>]*>", options)
        chosen = [o for o in options if " selected" in o] or ([] if tag.count("[]") else options[:1])
        if chosen:
            fields[attr(tag, "name")] = attr(chosen[0], "value") or ""
    for tag in re.findall(r"<(?:input|textarea)\b[^>]*>", re.sub(r"<select\b.*?</select>", "", body, flags=re.S)):
        kind = attr(tag, "type")
        if attr(tag, "name") and kind not in ("submit", "button", "file", "image") and (kind not in ("checkbox", "radio") or " checked" in tag):
            fields[attr(tag, "name")] = attr(tag, "value") or ""
    return html.unescape(found.group(1)), fields


class Session:
    def __init__(self, user):
        self.jar = http.cookiejar.CookieJar()
        self.http = urllib.request.build_opener(urllib.request.HTTPCookieProcessor(self.jar))
        _, page = self.get("/login/index.php")
        password = re.search(rf"\b{user} / (\S+)", re.sub(r"<[^>]+>", " ", page)).group(1)  # listed on the login page
        token = re.search(r'name="logintoken" value="([^"]+)"', page).group(1)
        if "login/index.php" in self.get("/login/index.php", {"username": user, "password": password, "logintoken": token})[0]:
            fail(f"could not log in as {user}")

    def get(self, url, data=None):
        body = urllib.parse.urlencode(data).encode() if data is not None else None
        with self.http.open(urllib.parse.urljoin(SITE + "/", url), body, timeout=60) as r:
            return r.geturl(), r.read().decode(errors="replace")

    def submit(self, path, action, **values):
        page_url, page = self.get(path)
        target, fields = form(page, action)
        return self.get(urllib.parse.urljoin(page_url, target), fields | values)


def main():
    args = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    args.add_argument("--quit-url", help="URL that ends the exam; Moodle shows an exit button linking to it")
    args.add_argument("--quit-password", help="password the student must enter to quit pantomim")
    args, started = args.parse_args(), time.time()

    print("[1/6] Logging in to the Moodle Sandbox as admin...", flush=True)
    admin = Session("admin")

    print('[2/6] Creating the course "Computer Networks (Class A)"...', flush=True)
    url, _ = admin.submit("/course/edit.php?category=1", "course/edit.php", fullname="Computer Networks (Class A)",
                          shortname=f"NET-A-{time.strftime('%H%M%S')}", saveanddisplay="1")
    course = re.search(r"[?&]id=(\d+)", url).group(1)

    print("[3/6] Enrolling the demo student...", flush=True)
    enrol = re.search(r"enrolid=(\d+)", admin.get(f"/enrol/instances.php?id={course}")[1]).group(1)
    student = re.search(r'<option value="(\d+)"[^>]*>[^<]*\(student@', admin.get(f"/enrol/manual/manage.php?enrolid={enrol}")[1]).group(1)
    # extendbase 4 = from now: a new course starts tomorrow, and so would the enrolment.
    admin.submit(f"/enrol/manual/manage.php?enrolid={enrol}", "manage.php", **{"addselect[]": student, "add": "1", "extendbase": "4"})

    print('[4/6] Creating "Quiz 1: OSI and TCP/IP", which requires Safe Exam Browser...', flush=True)
    seb = {"seb_requiresafeexambrowser": "1", "seb_showsebdownloadlink": "1"}
    if args.quit_url:
        seb |= {"seb_linkquitseb": args.quit_url, "seb_userconfirmquit": "1"}
    if args.quit_password:
        seb |= {"seb_quitpassword": args.quit_password, "seb_allowuserquitseb": "1"}
    url, _ = admin.submit(f"/course/modedit.php?add=quiz&course={course}&section=0&return=0", "modedit.php",
                          name="Quiz 1: OSI and TCP/IP", submitbutton="1", **seb,
                          **{"introeditor[text]": "This quiz requires Safe Exam Browser. Choose the best answer for each question."})
    cmid = (re.search(r"quiz/view\.php\?id=(\d+)", url) or fail("the quiz was not created")).group(1)

    # Moodle only checks for SEB on a quiz that can be attempted, so it needs questions.
    for number, (text, answers) in enumerate(QUESTIONS, 1):
        print(f"[5/6] Adding question {number} of {len(QUESTIONS)}...", flush=True)
        add = re.search(r'href="[^"]*(/question/bank/editquestion/addquestion\.php[^"]*)"', admin.get(f"/mod/quiz/edit.php?cmid={cmid}")[1])
        _, chooser = form(admin.get(html.unescape(add.group(1)))[1], "question.php")
        query = urllib.parse.urlencode({k: v for k, v in chooser.items() if k != "sesskey"} | {"qtype": "multichoice"})
        url, _ = admin.submit(f"/question/bank/editquestion/question.php?{query}", "question.php", name=f"Q{number}", submitbutton="1",
                              **{"questiontext[text]": text, "fraction[0]": "1.0"}, **{f"answer[{i}][text]": a for i, a in enumerate(answers)})
        if "mod/quiz/edit.php" not in url:
            fail(f"question {number} was not added")

    print("[6/6] Downloading the quiz's .seb file and logging the student in...", flush=True)
    student = Session("student")
    config = student.http.open(f"{SITE}/mod/quiz/accessrule/seb/config.php?cmid={cmid}", timeout=60).read()
    if not config.lstrip().startswith(b"<?xml"):
        fail("Moodle did not return a .seb file")
    profile = OUT / "data" / "io.github.dirgaydtm.pantomim"
    profile.mkdir(parents=True, exist_ok=True)
    (OUT / "quiz.seb").write_bytes(config)
    # WebKitGTK's cookies.txt; session cookies need an expiry to be loaded.
    (profile / "cookies").write_text("".join(
        f"{c.domain}\t{str(c.domain.startswith('.')).upper()}\t{c.path}\t{str(c.secure).upper()}\t{c.expires or int(time.time()) + 3600}\t{c.name}\t{c.value}\tNone\n"
        for c in student.jar))

    print(f"\nReady in {time.time() - started:.0f}s: {SITE}/mod/quiz/view.php?id={cmid}\nOpen it in pantomim as the demo student:\n")
    if shutil.which("pantomim"):
        print(f"  XDG_DATA_HOME={OUT / 'data'} pantomim {OUT / 'quiz.seb'}\n\nor, from this repository:\n")
    print(f"  XDG_DATA_HOME={OUT / 'data'} bun run tauri dev -- -- {OUT / 'quiz.seb'}")


main()
