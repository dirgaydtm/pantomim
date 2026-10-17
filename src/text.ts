// Texts from SafeExamBrowser.I18n/Data/{id,en}.xml, word for word, chosen like SEB by the system language.
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
		// Not from SEB, which starts with its default configuration when it gets no argument.
		NoConfiguration:
			'Buka lewat tombol "Launch Safe Exam Browser" di Moodle atau dengan membuka file .seb.',
		NoConfigurationTitle: "",
		MessageBox_BrowserQuitUrlConfirmation:
			"Aplikasi peramban telah mendeteksi URL keluar! Apakah Anda ingin menghentikan SEB sekarang?",
		MessageBox_BrowserQuitUrlConfirmationTitle: "URL Keluar Terdeteksi",
		MessageBox_InvalidQuitPassword:
			"SEB hanya dapat dihentikan dengan memasukkan kata sandi keluar yang benar.",
		MessageBox_InvalidQuitPasswordTitle: "Kata Sandi Keluar Tidak Valid",
		MessageBox_Quit: "Apakah Anda ingin keluar dari SEB?",
		MessageBox_QuitTitle: "Keluar?",
		PasswordDialog_Cancel: "Batal",
		PasswordDialog_Confirm: "Konfirmasi",
		PasswordDialog_QuitPasswordRequired:
			"Masukkan kata sandi keluar untuk mengakhiri SEB:",
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
			'Open it with the "Launch Safe Exam Browser" button in Moodle or by opening a .seb file.',
		NoConfigurationTitle: "",
		MessageBox_BrowserQuitUrlConfirmation:
			"The browser application has detected a quit URL! Would you like to terminate SEB now?",
		MessageBox_BrowserQuitUrlConfirmationTitle: "Quit URL Detected",
		MessageBox_InvalidQuitPassword:
			"SEB can only be terminated by entering the correct quit password.",
		MessageBox_InvalidQuitPasswordTitle: "Invalid Quit Password",
		MessageBox_Quit: "Do you want to quit SEB?",
		MessageBox_QuitTitle: "Quit?",
		PasswordDialog_Cancel: "Cancel",
		PasswordDialog_Confirm: "Confirm",
		PasswordDialog_QuitPasswordRequired:
			"Please enter the quit password in order to terminate SEB:",
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
