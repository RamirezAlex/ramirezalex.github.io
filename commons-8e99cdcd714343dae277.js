(self.webpackChunkgatsby_starter_default=self.webpackChunkgatsby_starter_default||[]).push([[351],{2993:function(e){var t="undefined"!=typeof Element,n="function"==typeof Map,r="function"==typeof Set,o="function"==typeof ArrayBuffer&&!!ArrayBuffer.isView;function i(e,a){if(e===a)return!0;if(e&&a&&"object"==typeof e&&"object"==typeof a){if(e.constructor!==a.constructor)return!1;var c,u,s,l;if(Array.isArray(e)){if((c=e.length)!=a.length)return!1;for(u=c;0!=u--;)if(!i(e[u],a[u]))return!1;return!0}if(n&&e instanceof Map&&a instanceof Map){if(e.size!==a.size)return!1;for(l=e.entries();!(u=l.next()).done;)if(!a.has(u.value[0]))return!1;for(l=e.entries();!(u=l.next()).done;)if(!i(u.value[1],a.get(u.value[0])))return!1;return!0}if(r&&e instanceof Set&&a instanceof Set){if(e.size!==a.size)return!1;for(l=e.entries();!(u=l.next()).done;)if(!a.has(u.value[0]))return!1;return!0}if(o&&ArrayBuffer.isView(e)&&ArrayBuffer.isView(a)){if((c=e.length)!=a.length)return!1;for(u=c;0!=u--;)if(e[u]!==a[u])return!1;return!0}if(e.constructor===RegExp)return e.source===a.source&&e.flags===a.flags;if(e.valueOf!==Object.prototype.valueOf)return e.valueOf()===a.valueOf();if(e.toString!==Object.prototype.toString)return e.toString()===a.toString();if((c=(s=Object.keys(e)).length)!==Object.keys(a).length)return!1;for(u=c;0!=u--;)if(!Object.prototype.hasOwnProperty.call(a,s[u]))return!1;if(t&&e instanceof Element)return!1;for(u=c;0!=u--;)if(("_owner"!==s[u]&&"__v"!==s[u]&&"__o"!==s[u]||!e.$$typeof)&&!i(e[s[u]],a[s[u]]))return!1;return!0}return e!=e&&a!=a}e.exports=function(e,t){try{return i(e,t)}catch(n){if((n.message||"").match(/stack|recursion/i))return console.warn("react-fast-compare cannot handle circular refs"),!1;throw n}}},5414:function(e,t,n){"use strict";n.d(t,{q:function(){return he}});var r,o,i,a,c=n(9324),u=n.n(c),s=n(4839),l=n.n(s),f=n(2993),p=n.n(f),d=n(7294),y=n(6494),h=n.n(y),m="bodyAttributes",b="htmlAttributes",T="titleAttributes",g={BASE:"base",BODY:"body",HEAD:"head",HTML:"html",LINK:"link",META:"meta",NOSCRIPT:"noscript",SCRIPT:"script",STYLE:"style",TITLE:"title"},v=(Object.keys(g).map((function(e){return g[e]})),"charset"),w="cssText",O="href",A="http-equiv",E="innerHTML",C="itemprop",S="name",k="property",j="rel",P="src",I="target",x={accesskey:"accessKey",charset:"charSet",class:"className",contenteditable:"contentEditable",contextmenu:"contextMenu","http-equiv":"httpEquiv",itemprop:"itemProp",tabindex:"tabIndex"},L="defaultTitle",M="defer",_="encodeSpecialCharacters",R="onChangeClientState",N="titleTemplate",D=Object.keys(x).reduce((function(e,t){return e[x[t]]=t,e}),{}),q=[g.NOSCRIPT,g.SCRIPT,g.STYLE],H="data-react-helmet",B="function"==typeof Symbol&&"symbol"==typeof Symbol.iterator?function(e){return typeof e}:function(e){return e&&"function"==typeof Symbol&&e.constructor===Symbol&&e!==Symbol.prototype?"symbol":typeof e},F=function(e,t){if(!(e instanceof t))throw new TypeError("Cannot call a class as a function")},U=function(){function e(e,t){for(var n=0;n<t.length;n++){var r=t[n];r.enumerable=r.enumerable||!1,r.configurable=!0,"value"in r&&(r.writable=!0),Object.defineProperty(e,r.key,r)}}return function(t,n,r){return n&&e(t.prototype,n),r&&e(t,r),t}}(),Y=Object.assign||function(e){for(var t=1;t<arguments.length;t++){var n=arguments[t];for(var r in n)Object.prototype.hasOwnProperty.call(n,r)&&(e[r]=n[r])}return e},z=function(e,t){var n={};for(var r in e)t.indexOf(r)>=0||Object.prototype.hasOwnProperty.call(e,r)&&(n[r]=e[r]);return n},W=function(e,t){if(!e)throw new ReferenceError("this hasn't been initialised - super() hasn't been called");return!t||"object"!=typeof t&&"function"!=typeof t?e:t},K=function(e){var t=!(arguments.length>1&&void 0!==arguments[1])||arguments[1];return!1===t?String(e):String(e).replace(/&/g,"&amp;").replace(/</g,"&lt;").replace(/>/g,"&gt;").replace(/"/g,"&quot;").replace(/'/g,"&#x27;")},Z=function(e){var t=G(e,g.TITLE),n=G(e,N);if(n&&t)return n.replace(/%s/g,(function(){return Array.isArray(t)?t.join(""):t}));var r=G(e,L);return t||r||void 0},V=function(e){return G(e,R)||function(){}},Q=function(e,t){return t.filter((function(t){return void 0!==t[e]})).map((function(t){return t[e]})).reduce((function(e,t){return Y({},e,t)}),{})},J=function(e,t){return t.filter((function(e){return void 0!==e[g.BASE]})).map((function(e){return e[g.BASE]})).reverse().reduce((function(t,n){if(!t.length)for(var r=Object.keys(n),o=0;o<r.length;o++){var i=r[o].toLowerCase();if(-1!==e.indexOf(i)&&n[i])return t.concat(n)}return t}),[])},$=function(e,t,n){var r={};return n.filter((function(t){return!!Array.isArray(t[e])||(void 0!==t[e]&&re("Helmet: "+e+' should be of type "Array". Instead found type "'+B(t[e])+'"'),!1)})).map((function(t){return t[e]})).reverse().reduce((function(e,n){var o={};n.filter((function(e){for(var n=void 0,i=Object.keys(e),a=0;a<i.length;a++){var c=i[a],u=c.toLowerCase();-1===t.indexOf(u)||n===j&&"canonical"===e[n].toLowerCase()||u===j&&"stylesheet"===e[u].toLowerCase()||(n=u),-1===t.indexOf(c)||c!==E&&c!==w&&c!==C||(n=c)}if(!n||!e[n])return!1;var s=e[n].toLowerCase();return r[n]||(r[n]={}),o[n]||(o[n]={}),!r[n][s]&&(o[n][s]=!0,!0)})).reverse().forEach((function(t){return e.push(t)}));for(var i=Object.keys(o),a=0;a<i.length;a++){var c=i[a],u=h()({},r[c],o[c]);r[c]=u}return e}),[]).reverse()},G=function(e,t){for(var n=e.length-1;n>=0;n--){var r=e[n];if(r.hasOwnProperty(t))return r[t]}return null},X=(r=Date.now(),function(e){var t=Date.now();t-r>16?(r=t,e(t)):setTimeout((function(){X(e)}),0)}),ee=function(e){return clearTimeout(e)},te="undefined"!=typeof window?window.requestAnimationFrame&&window.requestAnimationFrame.bind(window)||window.webkitRequestAnimationFrame||window.mozRequestAnimationFrame||X:n.g.requestAnimationFrame||X,ne="undefined"!=typeof window?window.cancelAnimationFrame||window.webkitCancelAnimationFrame||window.mozCancelAnimationFrame||ee:n.g.cancelAnimationFrame||ee,re=function(e){return console&&"function"==typeof console.warn&&console.warn(e)},oe=null,ie=function(e,t){var n=e.baseTag,r=e.bodyAttributes,o=e.htmlAttributes,i=e.linkTags,a=e.metaTags,c=e.noscriptTags,u=e.onChangeClientState,s=e.scriptTags,l=e.styleTags,f=e.title,p=e.titleAttributes;ue(g.BODY,r),ue(g.HTML,o),ce(f,p);var d={baseTag:se(g.BASE,n),linkTags:se(g.LINK,i),metaTags:se(g.META,a),noscriptTags:se(g.NOSCRIPT,c),scriptTags:se(g.SCRIPT,s),styleTags:se(g.STYLE,l)},y={},h={};Object.keys(d).forEach((function(e){var t=d[e],n=t.newTags,r=t.oldTags;n.length&&(y[e]=n),r.length&&(h[e]=d[e].oldTags)})),t&&t(),u(e,y,h)},ae=function(e){return Array.isArray(e)?e.join(""):e},ce=function(e,t){void 0!==e&&document.title!==e&&(document.title=ae(e)),ue(g.TITLE,t)},ue=function(e,t){var n=document.getElementsByTagName(e)[0];if(n){for(var r=n.getAttribute(H),o=r?r.split(","):[],i=[].concat(o),a=Object.keys(t),c=0;c<a.length;c++){var u=a[c],s=t[u]||"";n.getAttribute(u)!==s&&n.setAttribute(u,s),-1===o.indexOf(u)&&o.push(u);var l=i.indexOf(u);-1!==l&&i.splice(l,1)}for(var f=i.length-1;f>=0;f--)n.removeAttribute(i[f]);o.length===i.length?n.removeAttribute(H):n.getAttribute(H)!==a.join(",")&&n.setAttribute(H,a.join(","))}},se=function(e,t){var n=document.head||document.querySelector(g.HEAD),r=n.querySelectorAll(e+"["+"data-react-helmet]"),o=Array.prototype.slice.call(r),i=[],a=void 0;return t&&t.length&&t.forEach((function(t){var n=document.createElement(e);for(var r in t)if(t.hasOwnProperty(r))if(r===E)n.innerHTML=t.innerHTML;else if(r===w)n.styleSheet?n.styleSheet.cssText=t.cssText:n.appendChild(document.createTextNode(t.cssText));else{var c=void 0===t[r]?"":t[r];n.setAttribute(r,c)}n.setAttribute(H,"true"),o.some((function(e,t){return a=t,n.isEqualNode(e)}))?o.splice(a,1):i.push(n)})),o.forEach((function(e){return e.parentNode.removeChild(e)})),i.forEach((function(e){return n.appendChild(e)})),{oldTags:o,newTags:i}},le=function(e){return Object.keys(e).reduce((function(t,n){var r=void 0!==e[n]?n+'="'+e[n]+'"':""+n;return t?t+" "+r:r}),"")},fe=function(e){var t=arguments.length>1&&void 0!==arguments[1]?arguments[1]:{};return Object.keys(e).reduce((function(t,n){return t[x[n]||n]=e[n],t}),t)},pe=function(e,t,n){switch(e){case g.TITLE:return{toComponent:function(){return e=t.title,n=t.titleAttributes,(r={key:e})[H]=!0,o=fe(n,r),[d.createElement(g.TITLE,o,e)];var e,n,r,o},toString:function(){return function(e,t,n,r){var o=le(n),i=ae(t);return o?"<"+e+' data-react-helmet="true" '+o+">"+K(i,r)+"</"+e+">":"<"+e+' data-react-helmet="true">'+K(i,r)+"</"+e+">"}(e,t.title,t.titleAttributes,n)}};case m:case b:return{toComponent:function(){return fe(t)},toString:function(){return le(t)}};default:return{toComponent:function(){return function(e,t){return t.map((function(t,n){var r,o=((r={key:n})[H]=!0,r);return Object.keys(t).forEach((function(e){var n=x[e]||e;if(n===E||n===w){var r=t.innerHTML||t.cssText;o.dangerouslySetInnerHTML={__html:r}}else o[n]=t[e]})),d.createElement(e,o)}))}(e,t)},toString:function(){return function(e,t,n){return t.reduce((function(t,r){var o=Object.keys(r).filter((function(e){return!(e===E||e===w)})).reduce((function(e,t){var o=void 0===r[t]?t:t+'="'+K(r[t],n)+'"';return e?e+" "+o:o}),""),i=r.innerHTML||r.cssText||"",a=-1===q.indexOf(e);return t+"<"+e+' data-react-helmet="true" '+o+(a?"/>":">"+i+"</"+e+">")}),"")}(e,t,n)}}}},de=function(e){var t=e.baseTag,n=e.bodyAttributes,r=e.encode,o=e.htmlAttributes,i=e.linkTags,a=e.metaTags,c=e.noscriptTags,u=e.scriptTags,s=e.styleTags,l=e.title,f=void 0===l?"":l,p=e.titleAttributes;return{base:pe(g.BASE,t,r),bodyAttributes:pe(m,n,r),htmlAttributes:pe(b,o,r),link:pe(g.LINK,i,r),meta:pe(g.META,a,r),noscript:pe(g.NOSCRIPT,c,r),script:pe(g.SCRIPT,u,r),style:pe(g.STYLE,s,r),title:pe(g.TITLE,{title:f,titleAttributes:p},r)}},ye=l()((function(e){return{baseTag:J([O,I],e),bodyAttributes:Q(m,e),defer:G(e,M),encode:G(e,_),htmlAttributes:Q(b,e),linkTags:$(g.LINK,[j,O],e),metaTags:$(g.META,[S,v,A,k,C],e),noscriptTags:$(g.NOSCRIPT,[E],e),onChangeClientState:V(e),scriptTags:$(g.SCRIPT,[P,E],e),styleTags:$(g.STYLE,[w],e),title:Z(e),titleAttributes:Q(T,e)}}),(function(e){oe&&ne(oe),e.defer?oe=te((function(){ie(e,(function(){oe=null}))})):(ie(e),oe=null)}),de)((function(){return null})),he=(o=ye,a=i=function(e){function t(){return F(this,t),W(this,e.apply(this,arguments))}return function(e,t){if("function"!=typeof t&&null!==t)throw new TypeError("Super expression must either be null or a function, not "+typeof t);e.prototype=Object.create(t&&t.prototype,{constructor:{value:e,enumerable:!1,writable:!0,configurable:!0}}),t&&(Object.setPrototypeOf?Object.setPrototypeOf(e,t):e.__proto__=t)}(t,e),t.prototype.shouldComponentUpdate=function(e){return!p()(this.props,e)},t.prototype.mapNestedChildrenToProps=function(e,t){if(!t)return null;switch(e.type){case g.SCRIPT:case g.NOSCRIPT:return{innerHTML:t};case g.STYLE:return{cssText:t}}throw new Error("<"+e.type+" /> elements are self-closing and can not contain children. Refer to our API for more information.")},t.prototype.flattenArrayTypeChildren=function(e){var t,n=e.child,r=e.arrayTypeChildren,o=e.newChildProps,i=e.nestedChildren;return Y({},r,((t={})[n.type]=[].concat(r[n.type]||[],[Y({},o,this.mapNestedChildrenToProps(n,i))]),t))},t.prototype.mapObjectTypeChildren=function(e){var t,n,r=e.child,o=e.newProps,i=e.newChildProps,a=e.nestedChildren;switch(r.type){case g.TITLE:return Y({},o,((t={})[r.type]=a,t.titleAttributes=Y({},i),t));case g.BODY:return Y({},o,{bodyAttributes:Y({},i)});case g.HTML:return Y({},o,{htmlAttributes:Y({},i)})}return Y({},o,((n={})[r.type]=Y({},i),n))},t.prototype.mapArrayTypeChildrenToProps=function(e,t){var n=Y({},t);return Object.keys(e).forEach((function(t){var r;n=Y({},n,((r={})[t]=e[t],r))})),n},t.prototype.warnOnInvalidChildren=function(e,t){return!0},t.prototype.mapChildrenToProps=function(e,t){var n=this,r={};return d.Children.forEach(e,(function(e){if(e&&e.props){var o=e.props,i=o.children,a=function(e){var t=arguments.length>1&&void 0!==arguments[1]?arguments[1]:{};return Object.keys(e).reduce((function(t,n){return t[D[n]||n]=e[n],t}),t)}(z(o,["children"]));switch(n.warnOnInvalidChildren(e,i),e.type){case g.LINK:case g.META:case g.NOSCRIPT:case g.SCRIPT:case g.STYLE:r=n.flattenArrayTypeChildren({child:e,arrayTypeChildren:r,newChildProps:a,nestedChildren:i});break;default:t=n.mapObjectTypeChildren({child:e,newProps:t,newChildProps:a,nestedChildren:i})}}})),t=this.mapArrayTypeChildrenToProps(r,t)},t.prototype.render=function(){var e=this.props,t=e.children,n=z(e,["children"]),r=Y({},n);return t&&(r=this.mapChildrenToProps(t,r)),d.createElement(o,r)},U(t,null,[{key:"canUseDOM",set:function(e){o.canUseDOM=e}}]),t}(d.Component),i.propTypes={base:u().object,bodyAttributes:u().object,children:u().oneOfType([u().arrayOf(u().node),u().node]),defaultTitle:u().string,defer:u().bool,encodeSpecialCharacters:u().bool,htmlAttributes:u().object,link:u().arrayOf(u().object),meta:u().arrayOf(u().object),noscript:u().arrayOf(u().object),onChangeClientState:u().func,script:u().arrayOf(u().object),style:u().arrayOf(u().object),title:u().string,titleAttributes:u().object,titleTemplate:u().string},i.defaultProps={defer:!0,encodeSpecialCharacters:!0},i.peek=o.peek,i.rewind=function(){var e=o.rewind();return e||(e=de({baseTag:[],bodyAttributes:{},encodeSpecialCharacters:!0,htmlAttributes:{},linkTags:[],metaTags:[],noscriptTags:[],scriptTags:[],styleTags:[],title:"",titleAttributes:{}})),e},a);he.renderStatic=he.rewind},4839:function(e,t,n){"use strict";var r,o=n(7294),i=(r=o)&&"object"==typeof r&&"default"in r?r.default:r;function a(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}var c=!("undefined"==typeof window||!window.document||!window.document.createElement);e.exports=function(e,t,n){if("function"!=typeof e)throw new Error("Expected reducePropsToState to be a function.");if("function"!=typeof t)throw new Error("Expected handleStateChangeOnClient to be a function.");if(void 0!==n&&"function"!=typeof n)throw new Error("Expected mapStateOnServer to either be undefined or a function.");return function(r){if("function"!=typeof r)throw new Error("Expected WrappedComponent to be a React component.");var u,s=[];function l(){u=e(s.map((function(e){return e.props}))),f.canUseDOM?t(u):n&&(u=n(u))}var f=function(e){var t,n;function o(){return e.apply(this,arguments)||this}n=e,(t=o).prototype=Object.create(n.prototype),t.prototype.constructor=t,t.__proto__=n,o.peek=function(){return u},o.rewind=function(){if(o.canUseDOM)throw new Error("You may only call rewind() on the server. Call peek() to read the current state.");var e=u;return u=void 0,s=[],e};var a=o.prototype;return a.UNSAFE_componentWillMount=function(){s.push(this),l()},a.componentDidUpdate=function(){l()},a.componentWillUnmount=function(){var e=s.indexOf(this);s.splice(e,1),l()},a.render=function(){return i.createElement(r,this.props)},o}(o.PureComponent);return a(f,"displayName","SideEffect("+function(e){return e.displayName||e.name||"Component"}(r)+")"),a(f,"canUseDOM",c),f}}},3544:function(e,t,n){"use strict";n.d(t,{Z:function(){return l}});var r=n(7294),o=n(1597),i=n(5414),a=n(5697),c=function(e){e.siteTitle;return r.createElement(r.Fragment,null)};c.propTypes={siteTitle:n.n(a)().string},c.defaultProps={siteTitle:""};var u=c,s=function(e){var t=e.isDarkMode,n=e.turnOffTheLight;return r.createElement("nav",null,r.createElement("button",{style:{color:t?"white":"black"},onClick:function(){n(!t)}},"Turn ",t?"on":"off"," the light ",t?"🌃":"🌆"))},l=function(e){var t=e.children;var n,a,c,l,f,p="undefined"!=typeof window&&window.localStorage.getItem("isDarkMode")&&window.localStorage.getItem("isDarkMode"),d=(n="isDarkMode",a=p,c=(0,r.useState)((function(){try{var e=window.localStorage.getItem(n);return e?JSON.parse(e):a}catch(t){return console.log(t),a}})),l=c[0],f=c[1],[l,function(e){try{var t=e instanceof Function?e(l):e;f(t),window.localStorage.setItem(n,JSON.stringify(t))}catch(r){console.log(r)}}]),y=d[0],h=d[1],m=(0,o.useStaticQuery)("3649515864");return r.createElement(r.Fragment,null,r.createElement(i.q,null,r.createElement("meta",{name:"viewport",content:"width=device-width, initial-scale=0.65"}),r.createElement("link",{rel:"icon",href:"images/ramirezalex.ico",type:"image/ico",sizes:"32x32"}),r.createElement("link",{rel:"stylesheet",href:"https://use.fontawesome.com/releases/v5.6.3/css/all.css",integrity:"sha384-UHRtZLI+pbxtHCWp1t77Bi1L4ZtiqrqD80Kn4Z8NTSRyMA2Fd33n5dQ8lWUE00s/",crossorigin:"anonymous"})),r.createElement(u,{siteTitle:m.site.siteMetadata.title}),r.createElement("main",{className:y?"dark-mode":"light-mode"},r.createElement(s,{turnOffTheLight:h,isDarkMode:y}),t,r.createElement("footer",null,"RamirezAlex - 2022")))}},9357:function(e,t,n){"use strict";var r=n(7294),o=n(5414),i=n(1597);function a(e){var t=e.description,n=e.lang,a=e.meta,c=e.title,u=(0,i.useStaticQuery)("712016698").site,s=t||u.siteMetadata.description;return r.createElement(o.q,{htmlAttributes:{lang:n},title:c,titleTemplate:"%s | "+u.siteMetadata.title,meta:[{name:"description",content:s},{property:"og:title",content:c},{property:"og:description",content:s},{property:"og:type",content:"website"},{name:"twitter:card",content:"summary"},{name:"twitter:creator",content:u.siteMetadata.author},{name:"twitter:title",content:c},{name:"twitter:description",content:s}].concat(a)})}a.defaultProps={lang:"en",meta:[],description:""},t.Z=a},7579:function(e,t,n){"use strict";var r=n(70);function o(){}function i(){}i.resetWarningCache=o,e.exports=function(){function e(e,t,n,o,i,a){if(a!==r){var c=new Error("Calling PropTypes validators directly is not supported by the `prop-types` package. Use PropTypes.checkPropTypes() to call them. Read more at http://fb.me/use-check-prop-types");throw c.name="Invariant Violation",c}}function t(){return e}e.isRequired=e;var n={array:e,bool:e,func:e,number:e,object:e,string:e,symbol:e,any:e,arrayOf:t,element:e,elementType:e,instanceOf:t,node:e,objectOf:t,oneOf:t,oneOfType:t,shape:t,exact:t,checkPropTypes:i,resetWarningCache:o};return n.PropTypes=n,n}},9324:function(e,t,n){e.exports=n(7579)()},70:function(e){"use strict";e.exports="SECRET_DO_NOT_PASS_THIS_OR_YOU_WILL_BE_FIRED"}}]);
//# sourceMappingURL=commons-8e99cdcd714343dae277.js.map