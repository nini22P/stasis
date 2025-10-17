import { useState, useEffect, useMemo } from 'react'
import {
  FluentProvider,
  webLightTheme,
  webDarkTheme,
  Input,
  Button,
  makeStyles,
  List,
  ListItem,
  DialogTitle,
  DialogSurface,
  DialogTrigger,
  Dialog,
  DialogBody,
  DialogContent,
  DialogActions,
  tokens,
  mergeClasses,
} from '@fluentui/react-components'
import type { Config, StoredConfig, Source } from './types'
import { getTranslations, languages } from './locales/i18n'
import { bundleIcon, DeleteFilled, DeleteRegular } from '@fluentui/react-icons'

const useStyles = makeStyles({
  container: {
    display: 'flex',
    flexDirection: 'column',
    height: '100vh',
    padding: '20px',
    boxSizing: 'border-box',
    gap: '8px',
  },
  list: {
    flexGrow: 1,
    overflow: 'auto',
  },
  listItem: {
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'center',
    borderRadius: '4px',
    padding: '8px',
    gap: '8px',
    '&:hover': {
      backgroundColor: tokens.colorSubtleBackgroundHover,
    },
  },
  listItemText: {
    overflow: 'hidden',
    textOverflow: 'ellipsis',
    whiteSpace: 'nowrap',
  },
  listItemSelected: {
    backgroundColor: tokens.colorSubtleBackgroundSelected,
    '@media (forced-colors:active)': {
      background: 'Highlight',
    },
  },
  buttonWrapper: {
    alignSelf: 'center',
  },
  inputGroup: {
    display: 'flex',
    gap: '8px',
    paddingBottom: '16px',
  },
  input: {
    flexGrow: 1,
  },
  footer: {
    marginTop: 'auto',
    display: 'flex',
    justifyContent: 'flex-end',
    gap: '10px',
  },
})

const DeleteIcon = bundleIcon(DeleteFilled, DeleteRegular)

export function App() {
  const styles = useStyles()

  const language = useMemo(() => navigator.language === 'zh-CN' ? languages.zhCn : languages.en, [])

  const t = useMemo(() => getTranslations(language), [language])

  const [config, setConfig] = useState<Config | null>(null)
  const [selectedUri, setSelectedUri] = useState<string>('')
  const [customUri, setCustomUri] = useState('')
  const [theme, setTheme] = useState(webLightTheme)

  useEffect(() => {
    const prefersDark = window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches
    // eslint-disable-next-line react-hooks/set-state-in-effect
    setTheme(prefersDark ? webDarkTheme : webLightTheme)

    window.loadConfig = (config: Config) => {
      console.log('loadConfig', config)

      setConfig(config)
      setSelectedUri(config.selected_uri)
    }

    window.ready()
  }, [])

  useEffect(() => {
    window.setTitle(t.title)
  }, [t.title])

  const handleChooseFile = async () => {
    const filePath = await window.choose()
    if (filePath) {
      setCustomUri(filePath)
    }
  }

  const handleClose = () => {
    window.close()
  }

  const handleSave = async () => {
    const customSources: Source[] = config?.custom_sources || []

    const configToSave: StoredConfig = {
      custom_sources: customSources,
      selected_uri: selectedUri,
    }

    const result = await window.save(JSON.stringify(configToSave))

    if (result === 'success') {
      window.close()
    }
  }

  const handleAddCustomSource = () => {
    if (!customUri) {
      return
    }

    const newCustomSources = [...config?.custom_sources || [], { name: customUri, uri: customUri }]
    setConfig((prev) => prev && ({ ...prev, custom_sources: newCustomSources }))
    setCustomUri('')
  }

  const handleRemoveCustomSource = (uri: string) => {
    const newCustomSources = config?.custom_sources.filter((source) => source.uri !== uri)
    if (!newCustomSources) {
      return
    }

    setConfig((prev) => prev && ({ ...prev, custom_sources: newCustomSources }))

    if (selectedUri === uri) {
      setSelectedUri('')
    }
  }

  if (!config) {
    return null
  }

  return (
    <FluentProvider theme={theme}>
      <div className={styles.container}>
        <List
          className={styles.list}
          selectionMode="single"
          navigationMode="composite"
          selectedItems={[selectedUri]}
          onSelectionChange={(_, data) => setSelectedUri(data.selectedItems[0].toString().trim())}
        >
          {
            config.builtin_sources.map((source) => (
              <ListItem
                className={mergeClasses(
                  styles.listItem,
                  selectedUri === source.uri && styles.listItemSelected
                )}
                key={source.uri}
                value={source.uri}
              >
                <div className={styles.listItemText} style={{ width: '100%' }}>
                  {source.name}
                </div>
              </ListItem>
            ))
          }
          {
            config.custom_sources.map((source) => (
              <ListItem
                className={mergeClasses(
                  styles.listItem,
                  selectedUri === source.uri && styles.listItemSelected
                )}
                key={source.uri}
                value={source.uri}
              >
                <div className={styles.listItemText} style={{ width: '100%' }}>
                  {source.name}
                </div>

                <div className={styles.buttonWrapper}>
                  <Button
                    aria-label={`${t.remove} ${source.name}`}
                    size="small"
                    icon={<DeleteIcon />}
                    onClick={(e) => {
                      e.stopPropagation()
                      handleRemoveCustomSource(source.uri)
                    }}
                  />
                </div>
              </ListItem>
            ))
          }
        </List>

        <div className={styles.footer}>
          <Dialog onOpenChange={() => setCustomUri('')}>
            <DialogTrigger disableButtonEnhancement>
              <Button>
                {t.add_custom_screen_saver}
              </Button>
            </DialogTrigger>
            <DialogSurface>
              <DialogBody>
                <DialogTitle>{t.add_custom_screen_saver}</DialogTitle>
                <DialogContent>
                  <div className={styles.inputGroup}>
                    <Input
                      className={styles.input}
                      placeholder={t.uri_placeholder}
                      value={customUri}
                      onChange={(_, data) => setCustomUri(data.value)}
                    />
                    <Button onClick={handleChooseFile}>
                      {t.choose_file}
                    </Button>
                  </div>
                </DialogContent>
                <DialogActions>
                  <DialogTrigger disableButtonEnhancement>
                    <Button appearance="secondary">{t.cancel}</Button>
                  </DialogTrigger >
                  <DialogTrigger disableButtonEnhancement>
                    <Button appearance="primary" onClick={handleAddCustomSource}>{t.add}</Button>
                  </DialogTrigger>
                </DialogActions>
              </DialogBody>
            </DialogSurface>
          </Dialog>
          <Button onClick={handleClose}>{t.cancel}</Button>
          <Button appearance="primary" onClick={handleSave}>
            {t.save}
          </Button>
        </div>
      </div>
    </FluentProvider>
  )
}