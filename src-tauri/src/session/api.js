(() => {
  const sha256 = (text) => {
    const fraction = (x) => ((x - Math.floor(x)) * 2 ** 32) | 0;
    const k = [];
    let h = [];
    for (let n = 2; k.length < 64; n++) {
      let prime = true;
      for (let d = 2; d * d <= n; d++) if (n % d === 0) prime = false;
      if (!prime) continue;
      if (h.length < 8) h.push(fraction(n ** (1 / 2)));
      k.push(fraction(n ** (1 / 3)));
    }

    const bytes = new TextEncoder().encode(text);
    const length = Math.ceil((bytes.length + 9) / 64) * 64;
    const data = new Uint8Array(length);
    data.set(bytes);
    data[bytes.length] = 0x80;
    const view = new DataView(data.buffer);
    view.setUint32(length - 4, bytes.length * 8);

    const rotr = (x, n) => (x >>> n) | (x << (32 - n));
    const w = [];
    for (let i = 0; i < length; i += 64) {
      for (let t = 0; t < 64; t++) {
        if (t < 16) {
          w[t] = view.getUint32(i + t * 4);
        } else {
          const [x, y] = [w[t - 15], w[t - 2]];
          const s0 = rotr(x, 7) ^ rotr(x, 18) ^ (x >>> 3);
          const s1 = rotr(y, 17) ^ rotr(y, 19) ^ (y >>> 10);
          w[t] = (w[t - 16] + s0 + w[t - 7] + s1) | 0;
        }
      }
      let [a, b, c, d, e, f, g, hh] = h;
      for (let t = 0; t < 64; t++) {
        const s1 = rotr(e, 6) ^ rotr(e, 11) ^ rotr(e, 25);
        const t1 = (hh + s1 + ((e & f) ^ (~e & g)) + k[t] + w[t]) | 0;
        const s0 = rotr(a, 2) ^ rotr(a, 13) ^ rotr(a, 22);
        const t2 = (s0 + ((a & b) ^ (a & c) ^ (b & c))) | 0;
        [hh, g, f, e, d, c, b, a] = [g, f, e, (d + t1) | 0, c, b, a, (t1 + t2) | 0];
      }
      h = [a, b, c, d, e, f, g, hh].map((x, i) => (x + h[i]) | 0);
    }
    return h.map((x) => (x >>> 0).toString(16).padStart(8, "0")).join("");
  };

  const url = location.href.split("#")[0];
  window.SafeExamBrowser = {
    version: "SEB_Windows___SEB_VERSION__",
    security: {
      browserExamKey: "",
      configKey: sha256(url + "__CONFIG_KEY__"),
      updateKeys: (callback) => callback(),
    },
  };
})();
