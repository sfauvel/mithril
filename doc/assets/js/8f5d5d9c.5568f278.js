"use strict";(self.webpackChunkmithril_doc=self.webpackChunkmithril_doc||[]).push([[3900],{76814:(e,i,t)=>{t.r(i),t.d(i,{assets:()=>l,contentTitle:()=>o,default:()=>c,frontMatter:()=>s,metadata:()=>d,toc:()=>a});var n=t(85893),r=t(11151);const s={title:"Mithril Keys Certification",authors:[{name:"Mithril Team"}],tags:["cardano","poolId","operational-certificate","kes-keys","mithril-keys","hybrid-mode"]},o=void 0,d={permalink:"/doc/dev-blog/2022/10/11/keys-certification-badge",source:"@site/blog/2022-10-11-keys-certification-badge/index.md",title:"Mithril Keys Certification",description:"Update 2022/12/19: The signer registration with declarative PoolId has been decommissioned.",date:"2022-10-11T00:00:00.000Z",formattedDate:"October 11, 2022",tags:[{label:"cardano",permalink:"/doc/dev-blog/tags/cardano"},{label:"poolId",permalink:"/doc/dev-blog/tags/pool-id"},{label:"operational-certificate",permalink:"/doc/dev-blog/tags/operational-certificate"},{label:"kes-keys",permalink:"/doc/dev-blog/tags/kes-keys"},{label:"mithril-keys",permalink:"/doc/dev-blog/tags/mithril-keys"},{label:"hybrid-mode",permalink:"/doc/dev-blog/tags/hybrid-mode"}],readingTime:2.39,hasTruncateMarker:!1,authors:[{name:"Mithril Team"}],frontMatter:{title:"Mithril Keys Certification",authors:[{name:"Mithril Team"}],tags:["cardano","poolId","operational-certificate","kes-keys","mithril-keys","hybrid-mode"]},unlisted:!1,prevItem:{title:"Mithril environments are updated",permalink:"/doc/dev-blog/2022/10/28/updated-environments"},nextItem:{title:"Mithril internal stores switch to SQLite.",permalink:"/doc/dev-blog/2022/09/14/sqlite-stores"}},l={authorsImageUrls:[void 0]},a=[{value:"The way the Mithril nodes handle the Certification of the SPOs is evolving",id:"the-way-the-mithril-nodes-handle-the-certification-of-the-spos-is-evolving",level:3},{value:"Upgrade a Mithril Signer running a previous version",id:"upgrade-a-mithril-signer-running-a-previous-version",level:4},{value:"Hybrid Certification mode in the Mithril network",id:"hybrid-certification-mode-in-the-mithril-network",level:4},{value:"How Keys Certification works",id:"how-keys-certification-works",level:4}];function h(e){const i={a:"a",code:"code",h3:"h3",h4:"h4",img:"img",li:"li",p:"p",pre:"pre",strong:"strong",ul:"ul",...(0,r.a)(),...e.components};return(0,n.jsxs)(n.Fragment,{children:[(0,n.jsxs)(i.p,{children:[(0,n.jsx)(i.strong,{children:"Update 2022/12/19"}),": The signer registration with ",(0,n.jsx)(i.strong,{children:"declarative"})," PoolId has been decommissioned."]}),"\n",(0,n.jsxs)(i.p,{children:[(0,n.jsx)(i.strong,{children:"Update 2022/11/30"}),": The signer registration with ",(0,n.jsx)(i.strong,{children:"declarative"})," PoolId has been deprecated and the ",(0,n.jsx)(i.strong,{children:"certified"})," PoolId is now the stable mode."]}),"\n",(0,n.jsx)(i.h3,{id:"the-way-the-mithril-nodes-handle-the-certification-of-the-spos-is-evolving",children:"The way the Mithril nodes handle the Certification of the SPOs is evolving"}),"\n",(0,n.jsxs)(i.p,{children:[(0,n.jsx)(i.strong,{children:"PR"}),": ",(0,n.jsx)(i.code,{children:"New STM registration procedure"})," ",(0,n.jsx)(i.a,{href:"https://github.com/input-output-hk/mithril/pull/433",children:"#433"})]}),"\n",(0,n.jsxs)(i.p,{children:[(0,n.jsx)(i.strong,{children:"Issues"}),": ",(0,n.jsx)(i.code,{children:"Implement Certification of the Mithril Verification Keys in Signer/Aggregator"})," ",(0,n.jsx)(i.a,{href:"https://github.com/input-output-hk/mithril/issues/455",children:"#455"})]}),"\n",(0,n.jsx)(i.p,{children:"We have released a new Mithril Signer Verification Keys Certification mechanism:"}),"\n",(0,n.jsxs)(i.ul,{children:["\n",(0,n.jsx)(i.li,{children:"Mithril Signer nodes running the previous version are still able to interact with the network without any further intervention"}),"\n",(0,n.jsx)(i.li,{children:"Mithril Signer nodes that are updated from a previous version must migrate some of their stores"}),"\n",(0,n.jsxs)(i.li,{children:["This mechanism is ",(0,n.jsx)(i.strong,{children:"experimental"})," and can be activated on demand by the SPOs"]}),"\n"]}),"\n",(0,n.jsx)(i.h4,{id:"upgrade-a-mithril-signer-running-a-previous-version",children:"Upgrade a Mithril Signer running a previous version"}),"\n",(0,n.jsxs)(i.p,{children:["The SPOs need to recompile their Signer node (as in this ",(0,n.jsx)(i.a,{href:"https://mithril.network/doc/manual/getting-started/run-signer-node",children:"guide"}),")."]}),"\n",(0,n.jsx)(i.p,{children:"The data stores of the node need to be updated by running the following command:"}),"\n",(0,n.jsx)(i.pre,{children:(0,n.jsx)(i.code,{className:"language-bash",children:"# The path to your data stores directory, which defaults to:\nDATA_STORES_DIRECTORY=/opt/mithril/stores\n\n# Run this command to upgrade your stores:\nsqlite3 ${DATA_STORES_DIRECTORY}/signer.sqlite3 \"UPDATE protocol_initializer SET value = json_object('stm_initializer', json(value), 'kes_signature', null) WHERE json_extract(value, '$.stm_initializer') IS NULL;\"\n"})}),"\n",(0,n.jsxs)(i.p,{children:["\u26a0\ufe0f"," If you don't update your data stores with this procedure, your node will not be able to register to the Aggregator temporarily. It should then take up to ",(0,n.jsx)(i.code,{children:"3"})," epochs before it is able to successfully register its individual signatures with the Aggregator."]}),"\n",(0,n.jsx)(i.h4,{id:"hybrid-certification-mode-in-the-mithril-network",children:"Hybrid Certification mode in the Mithril network"}),"\n",(0,n.jsx)(i.p,{children:"From now, SPOs can either run their node by:"}),"\n",(0,n.jsxs)(i.ul,{children:["\n",(0,n.jsxs)(i.li,{children:["\n",(0,n.jsxs)(i.p,{children:[(0,n.jsxs)(i.strong,{children:["Declaring their Cardano ",(0,n.jsx)(i.code,{children:"PoolId"})]}),":"]}),"\n",(0,n.jsxs)(i.ul,{children:["\n",(0,n.jsx)(i.li,{children:"This is the mode that all nodes were running prior to this release"}),"\n",(0,n.jsxs)(i.li,{children:["This mode is still the ",(0,n.jsx)(i.strong,{children:"stable"})," mode"]}),"\n",(0,n.jsx)(i.li,{children:"We intend to deprecate this mode in the near future"}),"\n"]}),"\n"]}),"\n",(0,n.jsxs)(i.li,{children:["\n",(0,n.jsxs)(i.p,{children:[(0,n.jsxs)(i.strong,{children:["Certifying their Cardano ",(0,n.jsx)(i.code,{children:"PoolId"})]}),":"]}),"\n",(0,n.jsxs)(i.ul,{children:["\n",(0,n.jsxs)(i.li,{children:["The certification is done by providing the Mithril Signer node with ",(0,n.jsx)(i.code,{children:"KES Secret Key Path"})," and ",(0,n.jsx)(i.code,{children:"Operational Certificate Path"})]}),"\n",(0,n.jsxs)(i.li,{children:["This is an ",(0,n.jsx)(i.strong,{children:"experimental"})," mode"]}),"\n",(0,n.jsxs)(i.li,{children:["We intend to make this mode the only way of providing a ",(0,n.jsx)(i.code,{children:"PoolId"})," in the near future"]}),"\n",(0,n.jsxs)(i.li,{children:["These ",(0,n.jsx)(i.code,{children:"PoolIds"})," will be marked with a ",(0,n.jsx)(i.code,{children:"Verified Signer"})," green badge in the ",(0,n.jsx)(i.a,{href:"https://mithril.network/explorer/",children:"Mithril Explorer"})," (",(0,n.jsx)(i.code,{children:"2"})," epochs after activating the Certification mode)"]}),"\n"]}),"\n"]}),"\n"]}),"\n",(0,n.jsxs)(i.p,{children:["The setup of a Mithril Signer node with these two modes is available in this ",(0,n.jsx)(i.a,{href:"https://mithril.network/doc/manual/getting-started/run-signer-node",children:"guide"}),"."]}),"\n",(0,n.jsxs)(i.p,{children:["Here is an example of the ",(0,n.jsx)(i.code,{children:"Verified Signer"})," badge displayed in the Certificate details popin:\n",(0,n.jsx)(i.img,{alt:"Verified Signer Badge",src:t(67074).Z+"",width:"550",height:"221"})]}),"\n",(0,n.jsx)(i.h4,{id:"how-keys-certification-works",children:"How Keys Certification works"}),"\n",(0,n.jsxs)(i.p,{children:["We rely on the Cardano ",(0,n.jsx)(i.code,{children:"KES Keys"})," and ",(0,n.jsx)(i.code,{children:"Operational Certificate"})," to be able to:"]}),"\n",(0,n.jsxs)(i.ul,{children:["\n",(0,n.jsxs)(i.li,{children:["Compute automatically the ",(0,n.jsx)(i.code,{children:"PoolId"})," from a valid ",(0,n.jsx)(i.code,{children:"Operational Certificate"})]}),"\n",(0,n.jsxs)(i.li,{children:["Sign the ",(0,n.jsx)(i.code,{children:"Mithril Signer Verification Key"})," with the ",(0,n.jsx)(i.code,{children:"KES Secret Key"})]}),"\n",(0,n.jsxs)(i.li,{children:["Verify that the ",(0,n.jsx)(i.code,{children:"Mithril Signer Verification Key"})," is associated to the owner of the pool"]}),"\n"]}),"\n",(0,n.jsx)(i.p,{children:(0,n.jsx)(i.img,{alt:"Keys Certification Schema",src:t(43757).Z+"",width:"1134",height:"881"})}),"\n",(0,n.jsxs)(i.p,{children:["Feel free to reach out to us on the ",(0,n.jsx)(i.a,{href:"https://discord.gg/5kaErDKDRq",children:"Discord channel"})," for questions and/or help."]})]})}function c(e={}){const{wrapper:i}={...(0,r.a)(),...e.components};return i?(0,n.jsx)(i,{...e,children:(0,n.jsx)(h,{...e})}):h(e)}},67074:(e,i,t)=>{t.d(i,{Z:()=>n});const n=t.p+"assets/images/badge-d830657d3818b56eb6d9dd154085e753.png"},43757:(e,i,t)=>{t.d(i,{Z:()=>n});const n=t.p+"assets/images/schema-2bb1f4c4f967eddf4006f3acb8dbcb88.jpg"},11151:(e,i,t)=>{t.d(i,{Z:()=>d,a:()=>o});var n=t(67294);const r={},s=n.createContext(r);function o(e){const i=n.useContext(s);return n.useMemo((function(){return"function"==typeof e?e(i):{...i,...e}}),[i,e])}function d(e){let i;return i=e.disableParentContext?"function"==typeof e.components?e.components(r):e.components||r:o(e.components),n.createElement(s.Provider,{value:i},e.children)}}}]);