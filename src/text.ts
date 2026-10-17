// Texts from SafeExamBrowser.I18n/Data/{id,en}.xml ("SEB" becomes "pantomim"), chosen like SEB by the system language.
const texts = {
	id: {
		MessageBox_InvalidConfigurationData:
			'Sumber konfigurasi "%%URI%%" berisi data yang tidak valid!',
		MessageBox_InvalidConfigurationDataTitle: "Kesalahan Konfigurasi",
		MessageBox_NotSupportedConfigurationResource:
			'Sumber konfigurasi "%%URI%%" tidak didukung!',
		MessageBox_NotSupportedConfigurationResourceTitle: "Kesalahan Konfigurasi",
		MessageBox_UnexpectedConfigurationError:
			'Terjadi kesalahan yang tidak diharapkan saat mencoba memuat sumber konfigurasi "%%URI%%"! Silakan baca file log untuk informasi lebih lanjut.',
		MessageBox_UnexpectedConfigurationErrorTitle: "Kesalahan Konfigurasi",
		PasswordDialog_SettingsPasswordRequired: "Masukkan kata sandi ujian:",
		PasswordDialog_SettingsPasswordRequiredTitle: "Kata Sandi Diperlukan",
		// pantomim's own: SEB without an argument starts with its default configuration instead.
		NoConfiguration:
			'Buka pantomim lewat tombol "Launch Safe Exam Browser" di Moodle atau dengan membuka file .seb.',
		NoConfigurationTitle: "pantomim",
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
		MessageBox_InvalidConfigurationData:
			'The configuration resource "%%URI%%" contains invalid data!',
		MessageBox_InvalidConfigurationDataTitle: "Configuration Error",
		MessageBox_NotSupportedConfigurationResource:
			'The configuration resource "%%URI%%" is not supported!',
		MessageBox_NotSupportedConfigurationResourceTitle: "Configuration Error",
		MessageBox_UnexpectedConfigurationError:
			'An unexpected error occurred while trying to load configuration resource "%%URI%%"! Please consult the log files for more information.',
		MessageBox_UnexpectedConfigurationErrorTitle: "Configuration Error",
		PasswordDialog_SettingsPasswordRequired: "Please enter the exam password:",
		PasswordDialog_SettingsPasswordRequiredTitle: "Password Required",
		NoConfiguration:
			'Open pantomim with the "Launch Safe Exam Browser" button in Moodle or by opening a .seb file.',
		NoConfigurationTitle: "pantomim",
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
