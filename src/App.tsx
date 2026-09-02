import { useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { playPop } from './sound'
import './App.css'
import './responsive.css'

type Activity = { id: number; start_time: string; end_time: string; description: string; productivity: string }
type Page = 'Vue d’ensemble' | 'Historique' | 'Statistiques' | 'Projets' | 'Paramètres'
const pages: Page[] = ['Vue d’ensemble', 'Historique', 'Statistiques', 'Projets', 'Paramètres']

export default function App() {
  const [name, setName] = useState(() => localStorage.getItem('timepulse-name') || '')
  const [nameDraft, setNameDraft] = useState('')
  const [page, setPage] = useState<Page>('Vue d’ensemble')
  const [activities, setActivities] = useState<Activity[]>([])
  const [soundEnabled, setSoundEnabled] = useState(() => localStorage.getItem('timepulse-pop-sound') !== 'off')
  const [interval, setIntervalValue] = useState(() => Number(localStorage.getItem('timepulse-reminder-interval')) || 30)
  const [showForm, setShowForm] = useState(false)
  const [activity, setActivity] = useState('')
  const [productivity, setProductivity] = useState('Productif')

  useEffect(() => { localStorage.setItem('timepulse-pop-sound', soundEnabled ? 'on' : 'off') }, [soundEnabled])
  useEffect(() => { localStorage.setItem('timepulse-reminder-interval', String(interval)) }, [interval])
  useEffect(() => { invoke<Activity[]>('list_activities').then(setActivities).catch(() => setActivities([])) }, [])
  const openForm = () => { setShowForm(true); playPop(soundEnabled) }
  const saveName = () => { if (nameDraft.trim()) { localStorage.setItem('timepulse-name', nameDraft.trim()); setName(nameDraft.trim()) } }
  const saveActivity = async () => { if (!activity.trim()) return; await invoke('save_activity', { description: activity.trim(), productivity }); setActivity(''); setShowForm(false); setActivities(await invoke<Activity[]>('list_activities')) }
  useEffect(() => { const timer = window.setTimeout(openForm, interval * 60_000); return () => window.clearTimeout(timer) }, [interval, soundEnabled])

  if (!name) return <main className="welcome"><div className="welcome-card"><div className="brand"><span className="brand-mark">◷</span>TimePulse</div><p className="eyebrow">PREMIÈRE CONFIGURATION</p><h1>Bienvenue dans TimePulse</h1><p>Comment devons-nous t’appeler ?</p><input autoFocus value={nameDraft} onChange={e => setNameDraft(e.target.value)} onKeyDown={e => e.key === 'Enter' && saveName()} placeholder="Ton prénom"/><button className="save" onClick={saveName}>Commencer</button></div></main>

  return <main className="shell"><aside className="sidebar"><div className="brand"><span className="brand-mark">◷</span>TimePulse</div><nav>{pages.map((item, i) => <button className={'nav-item ' + (page === item ? 'active' : '')} onClick={() => setPage(item)} key={item}>{['⌂', '◷', '▦', '●', '⚙'][i]} {item}</button>)}</nav><div className="sidebar-bottom"><div className="status"><i/> Rappels actifs <b>{interval} min</b></div></div></aside><section className="content"><header className="topbar"><div><p className="eyebrow">MERCREDI 2 SEPTEMBRE 2026</p><h1>Bonjour, {name}</h1></div><button className="avatar">{name.slice(0, 2).toUpperCase()}</button></header>{page === 'Vue d’ensemble' && <><div className="date-row"><strong>Aujourd’hui</strong><button className="add-button" onClick={openForm}>+ Ajouter une activité</button></div><section className="metrics"><div className="metric primary"><span>Temps suivi</span><strong>0 h 00</strong><small>Aucune activité enregistrée</small></div><div className="metric"><span className="green">Productif</span><strong>0 h 00</strong><small>0 % du temps suivi</small></div><div className="metric"><span className="orange">Neutre</span><strong>0 h 00</strong><small>0 % du temps suivi</small></div><div className="metric"><span className="red">Temps perdu</span><strong>0 h 00</strong><small>0 % du temps suivi</small></div></section><section className="card timeline"><div className="card-head"><div><p className="eyebrow">VOTRE JOURNÉE</p><h2>Timeline</h2></div></div>{activities.length ? activities.map(item => <div className="entry" key={item.id}><time>{item.start_time} – {item.end_time}</time><div className="entry-dot"/><div className="entry-info"><strong>{item.description}</strong><span>TimePulse</span></div><em className="tag green-bg">{item.productivity}</em></div>) : <p className="empty">Aucune activité aujourd’hui. Ajoute ta première activité pour commencer le suivi.</p>}</section></>}{page !== 'Vue d’ensemble' && <section className="card page-placeholder"><p className="eyebrow">TIMEPULSE</p><h2>{page}</h2><p>Cette section est prête à accueillir tes données réelles. Aucune donnée fictive n’est affichée.</p>{page === 'Paramètres' && <><label className="toggle-row"><span>Son “pop” à l’ouverture</span><input type="checkbox" checked={soundEnabled} onChange={e => setSoundEnabled(e.target.checked)}/></label><label className="toggle-row"><span>Intervalle</span><select value={interval} onChange={e => setIntervalValue(Number(e.target.value))}><option value="15">15 minutes</option><option value="30">30 minutes</option><option value="45">45 minutes</option><option value="60">60 minutes</option></select></label><button className="link-button" onClick={() => { localStorage.removeItem('timepulse-name'); setName('') }}>Modifier le prénom</button></>}</section>}{showForm && <div className="overlay"><div className="modal"><button className="close" onClick={() => setShowForm(false)}>×</button><p className="eyebrow">NOUVELLE ACTIVITÉ</p><h2>Qu’as-tu fait récemment ?</h2><textarea autoFocus placeholder="Qu’as-tu fait ?" value={activity} onChange={e => setActivity(e.target.value)}/><div className="mood-buttons">{['Productif', 'Neutre', 'Temps perdu'].map(x => <button className={productivity === x ? 'selected' : ''} onClick={() => setProductivity(x)} key={x}>{x}</button>)}</div><button className="save" onClick={saveActivity}>Enregistrer</button></div></div>}</section></main>
}
