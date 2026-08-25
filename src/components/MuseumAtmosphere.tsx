import { useEffect, useRef } from 'react'

type Theme = 'night' | 'paper' | 'amber'

export default function MuseumAtmosphere({ reducedMotion, theme }: { reducedMotion: boolean; theme: Theme }) {
  const canvasRef = useRef<HTMLCanvasElement>(null)
  useEffect(() => {
    const canvas = canvasRef.current
    if (!canvas) return
    const ctx = canvas.getContext('2d')
    if (!ctx) return
    const dpr = Math.min(window.devicePixelRatio || 1, 1.5)
    const palette = theme === 'paper' ? ['#2e9c8f', '#d49a5d', '#86c9bc'] : theme === 'amber' ? ['#e8bd70', '#d99864', '#9b5d57'] : ['#8e7cff', '#4d8de0', '#9bb9ff']
    const particles = Array.from({ length: 90 }, (_, index) => ({
      x: Math.random(), y: Math.random(), r: 0.6 + Math.random() * 1.8, speed: 0.00008 + Math.random() * 0.00018, phase: index * 0.7,
    }))
    let frame = 0
    let raf = 0
    const resize = () => {
      const rect = canvas.getBoundingClientRect()
      canvas.width = rect.width * dpr
      canvas.height = rect.height * dpr
      ctx.setTransform(dpr, 0, 0, dpr, 0, 0)
    }
    const draw = () => {
      const rect = canvas.getBoundingClientRect()
      const w = rect.width
      const h = rect.height
      ctx.clearRect(0, 0, w, h)
      const now = reducedMotion ? 0 : frame
      const gradient = ctx.createLinearGradient(0, h, w, 0)
      gradient.addColorStop(0, `${palette[0]}12`)
      gradient.addColorStop(.5, `${palette[1]}0a`)
      gradient.addColorStop(1, `${palette[2]}18`)
      ctx.fillStyle = gradient
      ctx.fillRect(0, 0, w, h)
      ctx.strokeStyle = `${palette[0]}28`
      ctx.lineWidth = 1
      for (let index = -8; index < 18; index += 1) {
        const x = index * 48 + ((now * .18) % 48)
        ctx.beginPath(); ctx.moveTo(x, h); ctx.lineTo(w / 2 + (x - w / 2) * .25, h * .46); ctx.stroke()
      }
      for (let index = 0; index < 8; index += 1) {
        const y = h * (.5 + index * .09) + ((now * .12) % 18)
        ctx.beginPath(); ctx.moveTo(0, y); ctx.lineTo(w, y); ctx.stroke()
      }
      palette.forEach((color, index) => {
        const x = w * (.2 + index * .3) + Math.sin(now * .006 + index) * 28
        const beam = ctx.createLinearGradient(x, h, x + (index - 1) * 80, 0)
        beam.addColorStop(0, `${color}00`); beam.addColorStop(.5, `${color}2b`); beam.addColorStop(1, `${color}00`)
        ctx.fillStyle = beam
        ctx.beginPath(); ctx.moveTo(x - 35, h); ctx.lineTo(x + 35, h); ctx.lineTo(x + 8, h * .08); ctx.lineTo(x - 8, h * .08); ctx.closePath(); ctx.fill()
      })
      particles.forEach((particle, index) => {
        const x = particle.x * w + Math.sin(now * .003 + particle.phase) * 9
        const y = ((particle.y + now * particle.speed) % 1) * h
        const color = palette[index % palette.length]
        ctx.fillStyle = color
        ctx.shadowColor = color
        ctx.shadowBlur = reducedMotion ? 0 : 10
        ctx.globalAlpha = .35 + (Math.sin(now * .01 + particle.phase) + 1) * .2
        ctx.beginPath(); ctx.arc(x, y, particle.r, 0, Math.PI * 2); ctx.fill()
      })
      ctx.globalAlpha = 1
      ctx.shadowBlur = 0
      if (!reducedMotion) { frame += 1; raf = requestAnimationFrame(draw) }
    }
    resize(); draw()
    window.addEventListener('resize', resize)
    return () => { window.removeEventListener('resize', resize); cancelAnimationFrame(raf) }
  }, [reducedMotion, theme])
  return <div className={`museum-atmosphere ${reducedMotion ? 'atmosphere-static' : ''}`} role="img" aria-label="抽象数字展品空间的粒子氛围" aria-hidden="true"><canvas ref={canvasRef} /></div>
}
