import en from './en'
import zhCn from './zhCn'

export interface Language {
  name: string
  code: string
}

export const languages = {
  en: { name: 'English', code: 'en' },
  zhCn: { name: '简体中文', code: 'zh-cn' },
}

export interface Translations {
  title: string
  add_custom_screen_saver: string
  uri_placeholder: string
  choose_file: string
  cancel: string
  save: string
  add: string
  remove: string
}

export function getTranslations(language: Language): Translations {
  switch (language.code) {
    case languages.en.code:
      return en
    case languages.zhCn.code:
      return zhCn
    default:
      return en
  }
}
