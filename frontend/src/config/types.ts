export interface Source {
  uri: string
  name: string
}

export interface Config {
  builtin_sources: Source[]
  custom_sources: Source[]
  selected_uri: string
}

export interface StoredConfig {
  custom_sources: Source[]
  selected_uri: string
}

declare global {
  interface Window {
    setTitle: (title: string) => void
    loadConfig: (config: Config) => void
    ready: () => void
    choose: () => Promise<string>
    quit: () => void
    save: (configJson: string) => Promise<'success' | 'failed'>
  }
}