// Texts from SafeExamBrowser.I18n/Data/{id,en}.xml ("SEB" becomes "pantomim"), chosen like SEB by the system language.
const texts = {
	id: {
		MessageBox_BrowserQuitUrlConfirmation:
			"Aplikasi peramban telah mendeteksi URL keluar! Apakah Anda ingin menghentikan pantomim sekarang?",
		MessageBox_BrowserQuitUrlConfirmationTitle: "URL Keluar Terdeteksi",
		MessageBox_InvalidQuitPassword:
			"pantomim hanya dapat dihentikan dengan memasukkan kata sandi keluar yang benar.",
		MessageBox_InvalidQuitPasswordTitle: "Kata Sandi Keluar Tidak Valid",
		MessageBox_Quit: "Apakah Anda ingin keluar dari pantomim?",
		MessageBox_QuitTitle: "Keluar?",
		PasswordDialog_Cancel: "Batal",
		PasswordDialog_Confirm: "Konfirmasi",
		PasswordDialog_QuitPasswordRequired:
			"Masukkan kata sandi keluar untuk mengakhiri pantomim:",
		PasswordDialog_QuitPasswordRequiredTitle: "Kata Sandi Diperlukan",
		Shell_QuitButton: "Hentikan Sesi",
		// Windows MessageBox buttons, which come from Windows itself, not SEB.
		Ok: "OK",
		Yes: "Ya",
		No: "Tidak",
	},
	en: {
		MessageBox_BrowserQuitUrlConfirmation:
			"The browser application has detected a quit URL! Would you like to terminate pantomim now?",
		MessageBox_BrowserQuitUrlConfirmationTitle: "Quit URL Detected",
		MessageBox_InvalidQuitPassword:
			"pantomim can only be terminated by entering the correct quit password.",
		MessageBox_InvalidQuitPasswordTitle: "Invalid Quit Password",
		MessageBox_Quit: "Do you want to quit pantomim?",
		MessageBox_QuitTitle: "Quit?",
		PasswordDialog_Cancel: "Cancel",
		PasswordDialog_Confirm: "Confirm",
		PasswordDialog_QuitPasswordRequired:
			"Please enter the quit password in order to terminate pantomim:",
		PasswordDialog_QuitPasswordRequiredTitle: "Password Required",
		Shell_QuitButton: "Terminate Session",
		Ok: "OK",
		Yes: "Yes",
		No: "No",
	},
};

export const indonesian = navigator.language.startsWith("id");
export const text = indonesian ? texts.id : texts.en;
export type TextKey = keyof typeof text;
