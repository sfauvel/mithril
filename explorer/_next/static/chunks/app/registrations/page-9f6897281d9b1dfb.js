(self.webpackChunk_N_E=self.webpackChunk_N_E||[]).push([[480],{3195:function(e,r,t){Promise.resolve().then(t.bind(t,2129))},2129:function(e,r,t){"use strict";t.r(r),t.d(r,{default:function(){return P}});var n=t(7437),i=t(4033),s=t(2265),a=t(3046),l=t(9792),o=t(7019),c=t(3839),d=t(2744),u=t.n(d),h=t(4061),g=t(5113),x=t(5956);let f=(0,t(4617).Z)("h4");f.displayName="DivStyledAsH4";let p=s.forwardRef(({className:e,bsPrefix:r,as:t=f,...i},s)=>(r=(0,x.vE)(r,"alert-heading"),(0,n.jsx)(t,{ref:s,className:u()(e,r),...i})));p.displayName="AlertHeading";var j=t(4226);let v=s.forwardRef(({className:e,bsPrefix:r,as:t=j.Z,...i},s)=>(r=(0,x.vE)(r,"alert-link"),(0,n.jsx)(t,{ref:s,className:u()(e,r),...i})));v.displayName="AlertLink";var b=t(2525),k=t(5754);let m=s.forwardRef((e,r)=>{let{bsPrefix:t,show:i=!0,closeLabel:s="Close alert",closeVariant:a,className:l,children:o,variant:c="primary",onClose:d,dismissible:f,transition:p=b.Z,...j}=(0,h.Ch)(e,{show:"onClose"}),v=(0,x.vE)(t,"alert"),m=(0,g.Z)(e=>{d&&d(!1,e)}),y=!0===p?b.Z:p,Z=(0,n.jsxs)("div",{role:"alert",...y?void 0:j,ref:r,className:u()(l,v,c&&`${v}-${c}`,f&&`${v}-dismissible`),children:[f&&(0,n.jsx)(k.Z,{onClick:m,"aria-label":s,variant:a}),o]});return y?(0,n.jsx)(y,{unmountOnExit:!0,...j,ref:void 0,in:i,children:Z}):i?Z:null});m.displayName="Alert";var y=Object.assign(m,{Link:v,Heading:p}),Z=t(8597),w=t(1387);let N=s.forwardRef(({bsPrefix:e,size:r,vertical:t=!1,className:i,role:s="group",as:a="div",...l},o)=>{let c=(0,x.vE)(e,"btn-group"),d=c;return t&&(d=`${c}-vertical`),(0,n.jsx)(a,{...l,ref:o,role:s,className:u()(i,d,r&&`${c}-${r}`)})});N.displayName="ButtonGroup";let S=s.forwardRef(({bsPrefix:e,variant:r,animation:t="border",size:i,as:s="div",className:a,...l},o)=>{e=(0,x.vE)(e,"spinner");let c=`${e}-${t}`;return(0,n.jsx)(s,{ref:o,...l,className:u()(a,c,i&&`${c}-${i}`,r&&`text-${r}`)})});S.displayName="Spinner";var M=t(4434),C=t(6704),E=t(105),_=t(9064),B=t(7585),A=t(1860),F=t(9957),$=t(825),R=t(7898);function P(){var e,r;let t=(0,a.I0)(),d=(0,i.useSearchParams)(),[u,h]=(0,s.useState)(!0),[g,x]=(0,s.useState)(void 0),[f,p]=(0,s.useState)(void 0),[j,v]=(0,s.useState)(void 0),[b,k]=(0,s.useState)(void 0),[m,C]=(0,s.useState)(void 0),[P,I]=(0,s.useState)([]),[L,T]=(0,s.useState)({stakesBreakdown:{},signersWeight:{}});(0,s.useEffect)(()=>{let e;let r=d.get(_.W),n=Number(d.get("epoch"));p(r),v(n),(0,o.checkUrl)(r)?Number.isInteger(n)||(e="invalidEpoch"):e="invalidAggregatorUrl",void 0===e?(fetch("".concat(r,"/signers/registered/").concat(n)).then(e=>200===e.status?e.json():{}).then(e=>{k(e.signing_at),I(e.registrations),T({stakesBreakdown:(0,l.Q2)(e.registrations),signersWeight:(0,l.Fw)(e.registrations)}),h(!1)}).catch(e=>{k(void 0),I([]),h(!1),console.error("Fetch registrations error:",e)}),fetch("".concat(r,"/epoch-settings")).then(e=>200===e.status?e.json():{}).then(e=>C(null==e?void 0:e.epoch)).catch(e=>{C(void 0),console.error("Fetch current epoch in epoch-settings error:",e)}),t((0,B.Q9)(r))):x(e)},[d]);let U=(0,s.useCallback)(e=>{let r=new URLSearchParams;return r.set("aggregator",f),r.set("epoch",e),"/registrations?".concat(r.toString())},[f]),W=U(j-1),O=U(m),D=U(j+1);if(void 0!==g){let e="";switch(g){case"invalidEpoch":e="The given epoch isn't an integer, please correct it and try again.";break;case"invalidAggregatorUrl":e="The given aggregator isn't a valid url, please correct it and try again.";break;default:e="Something went wrong"}return(0,n.jsxs)(c.Z,{gap:3,children:[(0,n.jsx)("h2",{children:"Registrations"}),(0,n.jsxs)(y,{variant:"danger",children:[(0,n.jsx)(y.Heading,{children:"Oh snap! You got an error!"}),(0,n.jsx)("p",{children:e})]})]})}return(0,n.jsxs)(c.Z,{gap:3,children:[(0,n.jsxs)("h2",{children:["Registrations"," ",(0,n.jsx)($.Z,{href:"".concat(f,"/signers/registered/").concat(j),variant:"outline-light",size:"sm"})]}),(0,n.jsx)(Z.Z,{children:(0,n.jsx)(w.Z,{children:(0,n.jsxs)("tbody",{children:[(0,n.jsxs)("tr",{children:[(0,n.jsx)("td",{children:(0,n.jsx)("strong",{children:"Aggregator:"})}),(0,n.jsx)("td",{children:f})]}),(0,n.jsxs)("tr",{children:[(0,n.jsx)("td",{children:(0,n.jsx)("strong",{children:"Registration epoch:"})}),(0,n.jsx)("td",{children:j})]}),(0,n.jsxs)("tr",{children:[(0,n.jsx)("td",{children:(0,n.jsx)("strong",{children:"Signing at epoch:"})}),(0,n.jsx)("td",{children:null!=b?b:"?"})]}),(0,n.jsxs)("tr",{children:[(0,n.jsx)("td",{children:(0,n.jsx)("strong",{children:"Number of signers:"})}),(0,n.jsx)("td",{children:null!==(e=null==P?void 0:P.length)&&void 0!==e?e:0})]}),(0,n.jsxs)("tr",{children:[(0,n.jsx)("td",{children:(0,n.jsx)("strong",{children:"Total stakes:"})}),(0,n.jsx)("td",{children:(0,n.jsx)(F.Z,{lovelace:null!==(r=null==P?void 0:P.reduce((e,r)=>e+r.stake,0))&&void 0!==r?r:0})})]})]})})}),(0,n.jsx)(Z.Z,{children:(0,n.jsx)("div",{children:Number.isInteger(j)&&(0,n.jsxs)(N,{children:[(0,n.jsxs)(A.Z,{href:W,children:["Previous Epoch (",j-1,")"]}),(0,n.jsxs)(A.Z,{href:O,disabled:void 0===m||m===j,children:["Current Epoch (",m,")"]}),(0,n.jsxs)(A.Z,{href:D,disabled:m<=j,children:["Next Epoch (",j+1,")"]})]})})}),u?(0,n.jsx)(S,{animation:"grow"}):void 0===P||0===P.length?(0,n.jsxs)(y,{variant:"info",children:[(0,n.jsxs)(y.Heading,{children:["No registrations for epoch ",j]}),(0,n.jsx)("p",{children:m===j?"The aggregator did not receive registrations yet for the current epoch.":m<j?"The epoch is in the future":"The aggregator may have pruned old registrations or wasn't running at this epoch."})]}):(0,n.jsxs)(Z.Z,{children:[(0,n.jsxs)(M.Z,{xs:12,sm:12,md:7,children:[(0,n.jsx)("h3",{children:"Signers"}),(0,n.jsx)(R.Z,{signers:P,aggregator:f,displayIndexes:!0})]}),(0,n.jsx)(M.Z,{xs:12,sm:12,md:5,children:(0,n.jsxs)(c.Z,{gap:3,children:[(0,n.jsx)("h3",{children:"Stakes breakdown"}),(0,n.jsx)(E.$Q,{data:L.stakesBreakdown}),(0,n.jsx)("h3",{children:"Signers weight"}),(0,n.jsx)(E.by,{data:L.signersWeight})]})})]})]})}C.kL.register(C.qi,C.uw,C.f$,C.ZL,C.Dx,C.u,C.De),(0,l.hZ)(C.kL)},9792:function(e,r,t){"use strict";t.d(r,{Fw:function(){return s},Q2:function(){return i},hZ:function(){return a}});var n=t(7019);function i(e){let r=e=>e/1e12,t=(null!=e?e:[]).map(e=>r(e.stake));return{labels:["< 1M₳","≥ 1M₳ < 10M₳","≥ 10M₳ < 25M₳","≥ 25M₳ < 50M₳","≥ 50M₳ < 75M₳","≥ 75M₳ < 100M₳","≥ 100M₳"],datasets:[{label:"Number of signers",data:[t.filter(e=>e<1).length,t.filter(e=>e>=1&&e<10).length,t.filter(e=>e>=10&&e<25).length,t.filter(e=>e>=25&&e<50).length,t.filter(e=>e>=50&&e<75).length,t.filter(e=>e>=75&&e<100).length,t.filter(e=>e>100).length]}]}}function s(e){let r=null!=e?e:[];return{labels:r.map(e=>e.party_id),datasets:[{label:"Stake (₳)",data:r.map(e=>(0,n.toAda)(e.stake))}]}}function a(e){let r=["rgba(255, 99, 132, 0.2)","rgba(255, 159, 64, 0.2)","rgba(255, 205, 86, 0.2)","rgba(75, 192, 192, 0.2)","rgba(54, 162, 235, 0.2)","rgba(153, 102, 255, 0.2)","rgba(201, 203, 207, 0.2)"],t=["rgb(255, 99, 132)","rgb(255, 159, 64)","rgb(255, 205, 86)","rgb(75, 192, 192)","rgb(54, 162, 235)","rgb(153, 102, 255)","rgb(201, 203, 207)"];e.defaults.plugins.legend.display=!1,e.defaults.elements.arc.backgroundColor=r,e.defaults.elements.arc.borderColor=t,e.defaults.elements.arc.borderWidth=1,e.defaults.elements.bar.backgroundColor=r,e.defaults.elements.bar.borderColor=t,e.defaults.elements.bar.borderWidth=1}},1860:function(e,r,t){"use strict";t.d(r,{Z:function(){return a}});var n=t(7437);t(2265);var i=t(1396),s=t.n(i);function a(e){let{href:r,children:t,className:i,disabled:a,...l}=e;return!0===a?i="".concat(i," disabled"):a=!1,(0,n.jsx)(s(),{href:r,"aria-disabled":a,className:"btn btn-primary link-underline-opacity-0 link-light ".concat(i),...l,children:t})}},4171:function(e,r,t){"use strict";t.d(r,{Z:function(){return l}});var n=t(7437);t(2265);var i=t(7322),s=t(7800),a=t(6716);function l(e){let{partyId:r}=e;return(0,n.jsxs)("span",{className:"text-break",children:[r,(0,n.jsx)(n.Fragment,{children:" "}),(0,n.jsx)(i.Z,{overlay:(0,n.jsx)(s.Z,{children:"Copy"}),children:(0,n.jsx)(a.Z,{variant:"link",onClick:function(){window.isSecureContext&&r&&navigator.clipboard.writeText(r).then(()=>{})},size:"md",className:"p-0",children:(0,n.jsx)("i",{className:"bi bi-copy",style:{color:"black"}})})})]})}t(7019)},3800:function(e,r,t){"use strict";t.d(r,{Z:function(){return h}});var n=t(7437),i=t(2265),s=t(3046),a=t(7585),l=t(7019),o=t(6691),c=t.n(o),d=t(7322),u=t(7800);function h(e){var r;let{aggregator:t,partyId:o,...h}=e,g=(0,s.v9)(e=>(0,a.Mj)(e,t,o)),[x,f]=(0,i.useState)(void 0);return(0,i.useEffect)(()=>{(null==g?void 0:g.network)?f((0,l.getCExplorerUrlForPool)(g.network,o)):f(void 0)},[o,g.network]),void 0!==x?(0,n.jsx)(n.Fragment,{children:(0,n.jsx)("a",{href:x,target:"_blank",className:"link-dark link-underline-light",children:(0,n.jsx)(d.Z,{overlay:(0,n.jsx)(u.Z,{children:"See in CExplorer"}),children:(0,n.jsxs)("span",{children:[(0,n.jsx)(c(),{src:"/explorer/cexplorer_logo.png",alt:"CExplorer Logo",style:{verticalAlign:"text-top"},width:20,height:20}),(0,n.jsx)(n.Fragment,{children:" "}),null!==(r=g.pool_ticker)&&void 0!==r?r:"Not available"]})})})}):(0,n.jsx)("span",{children:g.pool_ticker})}},825:function(e,r,t){"use strict";t.d(r,{Z:function(){return l}});var n=t(7437);t(2265);var i=t(7322),s=t(7800),a=t(6716);function l(e){return(0,n.jsx)(i.Z,{overlay:(0,n.jsx)(s.Z,{children:"Raw JSON"}),children:(0,n.jsx)(a.Z,{variant:"outline-secondary",target:"_blank",...e,children:(0,n.jsx)("i",{className:"bi bi-filetype-json",style:{color:"black"}})})})}},7898:function(e,r,t){"use strict";t.d(r,{Z:function(){return c}});var n=t(7437);t(2265);var i=t(1387),s=t(4171),a=t(657),l=t(3800),o=t(9957);function c(e){let{signers:r,aggregator:t,displayIndexes:c,...d}=e;return(0,n.jsxs)(i.Z,{responsive:!0,striped:!0,...d,children:[(0,n.jsx)("thead",{children:(0,n.jsxs)("tr",{children:[void 0!==c&&(0,n.jsx)("th",{children:"#"}),(0,n.jsx)("th",{children:"Party id"}),(0,n.jsx)("th",{children:"Pool Ticker"}),(0,n.jsx)("th",{style:{textAlign:"end"},children:"Stake"})]})}),(0,n.jsx)("tbody",{children:r.map((e,r)=>(0,n.jsxs)("tr",{children:[!0===c&&(0,n.jsx)("td",{children:r}),(0,n.jsxs)("td",{children:[(0,n.jsx)(a.Z,{tooltip:"Verified Signer"})," ",(0,n.jsx)(s.Z,{partyId:e.party_id})]}),(0,n.jsx)("td",{children:(0,n.jsx)(l.Z,{aggregator:t,partyId:e.party_id})}),(0,n.jsx)("td",{style:{textAlign:"end"},children:(0,n.jsx)(o.Z,{lovelace:e.stake})})]},e.party_id))})]})}},9957:function(e,r,t){"use strict";t.d(r,{Z:function(){return l}});var n=t(7437);t(2265);var i=t(7322),s=t(7800),a=t(7019);function l(e){let{lovelace:r}=e;return(0,n.jsx)(i.Z,{overlay:(0,n.jsxs)(s.Z,{children:[(0,a.formatCurrency)((0,a.toAda)(r),20),"₳"]}),children:(0,n.jsx)("span",{children:(0,a.formatStake)(r)})})}},657:function(e,r,t){"use strict";t.d(r,{Z:function(){return a}});var n=t(7437);t(2265);var i=t(7322),s=t(7800);function a(e){return(0,n.jsx)(i.Z,{overlay:(0,n.jsx)(s.Z,{children:e.tooltip}),children:(0,n.jsx)("a",{href:"#",className:"badge bg-success",children:(0,n.jsx)("i",{className:"bi bi-shield-lock"})})})}},9064:function(e,r,t){"use strict";t.d(r,{W:function(){return n}});let n="aggregator"},7585:function(e,r,t){"use strict";t.d(r,{Mj:function(){return o},Q9:function(){return a},Ux:function(){return s}});var n=t(9686),i=t(8742);let s=(0,n.oM)({name:"pools",initialState:{list:[]},reducers:{},extraReducers:e=>e.addCase(a.fulfilled,(e,r)=>{if(r.payload.keep_cached_data)return;let t=l(e,r.payload.aggregator);t?(t.network=r.payload.network,t.pools=r.payload.pools,t.date=r.payload.date):e.list.push({aggregator:r.payload.aggregator,date:r.payload.date,network:r.payload.network,pools:r.payload.pools})})}),a=(0,n.hg)("pools/updateForAggregator",(e,r)=>{var t;let n=l(r.getState().pools,e),i=Date.now();return i-(null!==(t=null==n?void 0:n.date)&&void 0!==t?t:0)>216e5?fetch("".concat(e,"/signers/tickers")).then(e=>200===e.status?e.json():{}).then(r=>{var t;return{aggregator:e,date:i,network:r.network,pools:null!==(t=r.signers)&&void 0!==t?t:[]}}):{keep_cached_data:!0}}),l=(e,r)=>e.list.find(e=>e.aggregator===r),o=(0,i.P1)([e=>e.pools,(e,r,t)=>({aggregator:r,poolId:t})],(e,r)=>{let t=l(e,r.aggregator),n=null==t?void 0:t.pools.find(e=>e.party_id===r.poolId);return{network:null==t?void 0:t.network,...n}});s.reducer},7019:function(e){let r=e=>e/1e6,t=function(e){let r=arguments.length>1&&void 0!==arguments[1]?arguments[1]:2;return e.toLocaleString(void 0,{maximumFractionDigits:r})};e.exports={checkUrl:function(e){try{return new URL(e),!0}catch(e){return!1}},formatStake:function(e){let n=r(e),i=[{suffix:"B",value:1e9},{suffix:"M",value:1e6},{suffix:"K",value:1e3},{suffix:"",value:1}].find(e=>Math.abs(n)>=e.value-.001);return i?"".concat(t(n/i.value)).concat(i.suffix,"₳"):"".concat(t(n),"₳")},toAda:r,formatCurrency:t,formatBytes:function(e){let r=arguments.length>1&&void 0!==arguments[1]?arguments[1]:2;if(0===e)return"0 Bytes";let t=Math.floor(Math.log(e)/Math.log(1024));return parseFloat((e/Math.pow(1024,t)).toFixed(r<0?0:r))+" "+["Bytes","KiB","MiB","GiB","TiB","PiB","EiB","ZiB","YiB"][t]},formatPartyId:function(e){return("string"==typeof e||e instanceof String)&&e.length>15?e.slice(0,10)+"…"+e.slice(-5):e},getCExplorerUrlForPool:function(e,r){let t;let n="cexplorer.io/pool/".concat(r);switch(e){case"mainnet":t="https://".concat(n);break;case"preprod":t="https://preprod.".concat(n);break;case"preview":t="https://preview.".concat(n)}return t}}}},function(e){e.O(0,[674,854,167,971,938,744],function(){return e(e.s=3195)}),_N_E=e.O()}]);