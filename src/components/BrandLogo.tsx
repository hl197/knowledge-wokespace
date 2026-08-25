import logoImage from '../assets/brand-logo.webp'

export default function BrandLogo({ size = 34 }: { size?: number }) {
  return <img className="brand-logo-image" src={logoImage} width={size} height={size} alt="知识工作台 Logo" />
}
