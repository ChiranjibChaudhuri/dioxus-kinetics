//! Brand assets for the Kinetics component gallery.

pub const KINETICS_LOGO_SVG: &str = r##"<svg width="720" height="220" viewBox="0 0 720 220" fill="none" xmlns="http://www.w3.org/2000/svg" role="img" aria-labelledby="title desc">
  <title id="title">dioxus-kinetics logo</title>
  <desc id="desc">An original kinetic glass mark with orbiting nodes and a dioxus-kinetics wordmark.</desc>
  <defs>
    <linearGradient id="panel" x1="70" y1="32" x2="184" y2="188" gradientUnits="userSpaceOnUse">
      <stop stop-color="#FFFFFF" stop-opacity="0.92"/>
      <stop offset="1" stop-color="#D8ECFF" stop-opacity="0.66"/>
    </linearGradient>
    <linearGradient id="orbitA" x1="34" y1="64" x2="216" y2="160" gradientUnits="userSpaceOnUse">
      <stop stop-color="#007AFF"/>
      <stop offset="0.52" stop-color="#42D3FF"/>
      <stop offset="1" stop-color="#24C46B"/>
    </linearGradient>
    <linearGradient id="orbitB" x1="46" y1="154" x2="214" y2="54" gradientUnits="userSpaceOnUse">
      <stop stop-color="#5856D6"/>
      <stop offset="0.5" stop-color="#0A84FF"/>
      <stop offset="1" stop-color="#64D2FF"/>
    </linearGradient>
    <filter id="softShadow" x="28" y="16" width="224" height="204" filterUnits="userSpaceOnUse" color-interpolation-filters="sRGB">
      <feFlood flood-opacity="0" result="BackgroundImageFix"/>
      <feColorMatrix in="SourceAlpha" type="matrix" values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 127 0" result="hardAlpha"/>
      <feOffset dy="14"/>
      <feGaussianBlur stdDeviation="18"/>
      <feColorMatrix type="matrix" values="0 0 0 0 0.08 0 0 0 0 0.14 0 0 0 0 0.22 0 0 0 0.18 0"/>
      <feBlend mode="normal" in2="BackgroundImageFix" result="effect1_dropShadow_1_1"/>
      <feBlend mode="normal" in="SourceGraphic" in2="effect1_dropShadow_1_1" result="shape"/>
    </filter>
  </defs>
  <rect width="720" height="220" rx="36" fill="#F6F8FB"/>
  <g filter="url(#softShadow)">
    <rect x="70" y="38" width="128" height="144" rx="32" fill="url(#panel)" stroke="#FFFFFF" stroke-width="2"/>
    <rect x="86" y="56" width="96" height="108" rx="24" fill="#FFFFFF" fill-opacity="0.28" stroke="#FFFFFF" stroke-opacity="0.7"/>
    <path d="M92 117C78 95 80 73 98 63C125 48 170 68 198 108C226 148 226 184 201 196C176 208 131 186 103 146" stroke="url(#orbitA)" stroke-width="12" stroke-linecap="round"/>
    <path d="M183 82C198 103 197 128 181 144C155 170 102 163 65 126C30 91 29 56 54 43C75 33 111 43 143 68" stroke="url(#orbitB)" stroke-width="10" stroke-linecap="round"/>
    <circle cx="196" cy="108" r="11" fill="#24C46B" stroke="#FFFFFF" stroke-width="4"/>
    <circle cx="64" cy="126" r="9" fill="#5856D6" stroke="#FFFFFF" stroke-width="4"/>
    <circle cx="133" cy="110" r="24" fill="#111827"/>
    <path d="M121 98H134C147 98 154 106 154 118C154 130 147 138 134 138H121V98ZM133 128C140 128 144 125 144 118C144 111 140 108 133 108H131V128H133Z" fill="white"/>
  </g>
  <text x="270" y="100" fill="#101722" font-family="Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, Segoe UI, sans-serif" font-size="42" font-weight="760" letter-spacing="0">dioxus-kinetics</text>
  <text x="272" y="138" fill="#526071" font-family="Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, Segoe UI, sans-serif" font-size="18" font-weight="520" letter-spacing="0">semantic glass UI for Dioxus SaaS apps</text>
</svg>"##;
