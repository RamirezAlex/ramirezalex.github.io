import React from 'react'

const Navbar = ({isDarkMode, turnOffTheLight}) => {
  const lightsOut = () => {
    turnOffTheLight(!isDarkMode)
  }

  return (
    <nav>
      <button
        style={ { color: isDarkMode ? 'white': 'black' } }
        onClick={ lightsOut }
      >
        Turn { isDarkMode? 'on': 'off'} the light { isDarkMode? '🌃': '🌆'}
      </button>
    </nav>
  )
}

export default Navbar
