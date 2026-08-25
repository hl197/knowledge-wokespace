import { useEffect, useRef } from 'react'

type Relation = { cardId: string; relationType: string; title?: string; type?: string }

export default function RelationSpace({ relations, onSelect }: { relations: Relation[]; onSelect: (relation: Relation) => void }) {
  const canvasRef = useRef<HTMLCanvasElement>(null)
  const nodes = relations.slice(0, 24)
  useEffect(() => {
    const canvas = canvasRef.current
    if (!canvas) return
    const ctx = canvas.getContext('2d')
    if (!ctx) return
    const dpr = Math.min(window.devicePixelRatio || 1, 2)
    const resize = () => {
      const rect = canvas.getBoundingClientRect()
      canvas.width = rect.width * dpr
      canvas.height = rect.height * dpr
      ctx.setTransform(dpr, 0, 0, dpr, 0, 0)
      draw()
    }
    const positions = nodes.map((node, index) => ({ node, angle: (index / Math.max(nodes.length, 1)) * Math.PI * 2, radius: 0.28 + (index % 3) * 0.08 }))
    let frame = 0
    const draw = () => {
      const rect = canvas.getBoundingClientRect()
      const w = rect.width
      const h = rect.height
      ctx.clearRect(0, 0, w, h)
      const cx = w / 2
      const cy = h / 2
      const time = frame * 0.004
      ctx.strokeStyle = 'color-mix(in srgb, var(--theme-accent) 28%, transparent)'
      ctx.lineWidth = 1
      positions.forEach((item, index) => {
        const next = positions[(index + 1) % positions.length]
        const x = cx + Math.cos(item.angle + time) * w * item.radius
        const y = cy + Math.sin(item.angle + time) * h * item.radius * 0.72
        const nx = cx + Math.cos(next.angle + time) * w * next.radius
        const ny = cy + Math.sin(next.angle + time) * h * next.radius * 0.72
        ctx.beginPath()
        ctx.moveTo(x, y)
        ctx.lineTo(nx, ny)
        ctx.stroke()
      })
      positions.forEach((item) => {
        const x = cx + Math.cos(item.angle + time) * w * item.radius
        const y = cy + Math.sin(item.angle + time) * h * item.radius * 0.72
        const color = item.node.type === '技能' ? '#8e7cff' : item.node.type === '知识' ? '#42d6c3' : '#e8bd70'
        ctx.shadowColor = color
        ctx.shadowBlur = 14
        ctx.fillStyle = color
        ctx.beginPath()
        ctx.arc(x, y, 6, 0, Math.PI * 2)
        ctx.fill()
        ctx.shadowBlur = 0
        ctx.fillStyle = 'rgba(255,255,255,.8)'
        ctx.font = '10px DM Sans, sans-serif'
        ctx.fillText((item.node.title || item.node.cardId).slice(0, 16), x + 10, y + 3)
      })
      ctx.fillStyle = 'var(--theme-accent)'
      ctx.shadowColor = 'var(--theme-accent)'
      ctx.shadowBlur = 22
      ctx.beginPath()
      ctx.arc(cx, cy, 18, 0, Math.PI * 2)
      ctx.fill()
      ctx.shadowBlur = 0
      if (nodes.length > 0) frame = requestAnimationFrame(draw)
    }
    resize()
    window.addEventListener('resize', resize)
    if (nodes.length > 0) frame = requestAnimationFrame(draw)
    return () => { window.removeEventListener('resize', resize); cancelAnimationFrame(frame) }
  }, [nodes])
  const pickNode = (event: React.MouseEvent<HTMLCanvasElement>) => {
    if (!nodes.length) return
    const rect = event.currentTarget.getBoundingClientRect()
    const x = event.clientX - rect.left
    const y = event.clientY - rect.top
    const index = Math.floor((x / Math.max(rect.width, 1)) * nodes.length) % nodes.length
    onSelect(nodes[index])
  }
  return <div className="relation-space" aria-label="关系空间星图" role="img"><canvas ref={canvasRef} onClick={pickNode} /><div className="relation-space-caption">关系空间 · {nodes.length} 个节点 · 点击节点查看关联</div></div>
}
