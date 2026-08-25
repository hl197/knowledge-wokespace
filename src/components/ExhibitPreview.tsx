import React from 'react'

export default function ExhibitPreview({ color, reducedMotion }: { color: string; reducedMotion: boolean }) {
  return (
    <div className={`exhibit-preview color-bends-card ${reducedMotion ? 'exhibit-static' : ''}`} style={{ '--exhibit-color': color } as React.CSSProperties} role="img" aria-label="可交互彩色数字展品预览">
      <div className="shine-border" aria-hidden="true" />
      <div className="color-bends-flow flow-one" />
      <div className="color-bends-flow flow-two" />
      <div className="color-bends-grid" />
      <div className="color-bends-glare" />
      <div className="color-bends-core"><span>✦</span><small>INTERACTIVE EXHIBIT</small></div>
    </div>
  )
}
