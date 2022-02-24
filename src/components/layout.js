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
  // Hook
  function useLocalStorage(key, initialValue) {
    // State to store our value
    // Pass initial state function to useState so logic is only executed once
    const [storedValue, setStoredValue] = useState(() => {
      try {
        // Get from local storage by key
        const item = window.localStorage.getItem(key);
        // Parse stored json or if none return initialValue
        return item ? JSON.parse(item) : initialValue;
      } catch (error) {
        // If error also return initialValue
        console.log(error);
        return initialValue;
      }
    });

    // Return a wrapped version of useState's setter function that ...
    // ... persists the new value to localStorage.
    const setValue = value => {
      try {
        // Allow value to be a function so we have same API as useState
        const valueToStore =
          value instanceof Function ? value(storedValue) : value;
        // Save state
        setStoredValue(valueToStore);
        // Save to local storage
        window.localStorage.setItem(key, JSON.stringify(valueToStore));
      } catch (error) {
        // A more advanced implementation would handle the error case
        console.log(error);
      }
    };

    return [storedValue, setValue];
  }

  const isBrowser = () => typeof window !== "undefined";
  const darkModeInitValue = isBrowser() && window.localStorage.getItem('isDarkMode') && window.localStorage.getItem('isDarkMode');
  const [isDarkMode,  turnOffTheLight] = useLocalStorage('isDarkMode', darkModeInitValue);

  const data = useStaticQuery(graphql`
    query SiteTitleQuery {
      site {
        siteMetadata {
          title
        }
      }
    }
  `);

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
          RamirezAlex - 2022
        </footer>
      </main>
    </>
  );
}

Layout.propTypes = {
  children: PropTypes.node.isRequired,
}

export default Layout
