// Texts from SafeExamBrowser.I18n/Data/{id,en}.xml, chosen like SEB by the system language.
const texts = {
  id: { Shell_QuitButton: "Hentikan Sesi" },
  en: { Shell_QuitButton: "Terminate Session" },
};

export const indonesian = navigator.language.startsWith("id");
export const text = indonesian ? texts.id : texts.en;
