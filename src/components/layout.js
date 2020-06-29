/**
 * Layout component that queries for data
 * with Gatsby's useStaticQuery component
 *
 * See: https://www.gatsbyjs.org/docs/use-static-query/
 */

import React, { useState } from "react"
import PropTypes from "prop-types"
import { useStaticQuery, graphql } from "gatsby"
import { Helmet } from "react-helmet"

import Header from "./header"
import Navbar from "./navbar"
import "./layout.css"

const Layout = ({ children }) => {
  const [isDarkMode,  turnOffTheLight] = useState(false)

  const data = useStaticQuery(graphql`
    query SiteTitleQuery {
      site {
        siteMetadata {
          title
        }
      }
    }
  `)

  return (
    <>
      <Helmet>
        <meta name="viewport" content="width=device-width, initial-scale=0.65" />
        <link rel="icon" href="images/ramirezalex.ico" type="image/ico" sizes="32x32" />
        <link rel="stylesheet" href="https://use.fontawesome.com/releases/v5.6.3/css/all.css" integrity="sha384-UHRtZLI+pbxtHCWp1t77Bi1L4ZtiqrqD80Kn4Z8NTSRyMA2Fd33n5dQ8lWUE00s/" crossorigin="anonymous"></link>
      </Helmet>
      <Header siteTitle={data.site.siteMetadata.title} />
      <main className={isDarkMode ? 'dark-mode' : 'light-mode' }>
        <Navbar turnOffTheLight={turnOffTheLight} isDarkMode={isDarkMode} />
        {children}
        <footer>
          RamirezAlex - 2020
        </footer>
      </main>
    </>
  )
}

Layout.propTypes = {
  children: PropTypes.node.isRequired,
}

export default Layout
