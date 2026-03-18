pub fn inject_script(html: &str, proxy: &str, origin: &str) -> String {
    let script = format!(
        r#"<script>
(function(){{
var P='{}',O='{}';
var B=function(){{throw new Error('Blocked for privacy')}};
Object.defineProperty(window,'RTCPeerConnection',{{value:B,writable:false,configurable:false}});
Object.defineProperty(window,'webkitRTCPeerConnection',{{value:B,writable:false,configurable:false}});
Object.defineProperty(window,'mozRTCPeerConnection',{{value:B,writable:false,configurable:false}});
Object.defineProperty(window,'WebSocket',{{value:B,writable:false,configurable:false}});
Object.defineProperty(window,'EventSource',{{value:B,writable:false,configurable:false}});
if(navigator.serviceWorker){{Object.defineProperty(navigator.serviceWorker,'register',{{value:function(){{return Promise.reject(new Error('Service Workers disabled'))}},writable:false}})}}
if(navigator.sendBeacon){{Object.defineProperty(navigator,'sendBeacon',{{value:function(){{return false}},writable:false,configurable:false}})}}
function px(u){{
if(!u||typeof u!=='string')return u;
if(u.startsWith('data:')||u.startsWith('blob:')||u.startsWith('javascript:')||u.includes('localhost:9060'))return u;
try{{var url=u.startsWith('http')?u:new URL(u,O).href;return P+encodeURIComponent(url)}}catch(e){{return u}}
}}
var _fetch=window.fetch;
var pxFetch=function(r,o){{var url=typeof r==='string'?r:(r&&r.url?r.url:r);var pu=px(url);if(typeof r==='string')return _fetch(pu,o);if(r&&typeof r==='object'){{var nr=new Request(pu,r);return _fetch(nr,o)}}return _fetch(r,o)}};
Object.defineProperty(window,'fetch',{{value:pxFetch,writable:false,configurable:false}});
var _xhr=XMLHttpRequest.prototype.open;
XMLHttpRequest.prototype.open=function(m,u,a,us,p){{return _xhr.call(this,m,px(u),a!==false,us,p)}};
Object.defineProperty(XMLHttpRequest.prototype,'open',{{value:XMLHttpRequest.prototype.open,writable:false,configurable:false}});
var _img=window.Image;
var PxImage=function(w,h){{var i=new _img(w,h);var _src=Object.getOwnPropertyDescriptor(HTMLImageElement.prototype,'src');Object.defineProperty(i,'src',{{set:function(v){{_src.set.call(this,px(v))}},get:function(){{return _src.get.call(this)}}}});return i}};
Object.defineProperty(window,'Image',{{value:PxImage,writable:false,configurable:false}});
if(window.Worker){{var _Worker=window.Worker;var PxWorker=function(u,o){{return new _Worker(px(u),o)}};Object.defineProperty(window,'Worker',{{value:PxWorker,writable:false,configurable:false}})}}
if(window.SharedWorker){{var _SW=window.SharedWorker;var PxSW=function(u,o){{return new _SW(px(u),o)}};Object.defineProperty(window,'SharedWorker',{{value:PxSW,writable:false,configurable:false}})}}
var _sS=Element.prototype.setAttribute;
Element.prototype.setAttribute=function(n,v){{if((n==='src'||n==='href'||n==='poster'||n==='data-src'||n==='srcset'||n==='ping'||n==='formaction')&&typeof v==='string'){{if(n==='srcset'){{v=v.split(',').map(function(p){{var ps=p.trim().split(/\s+/);if(ps[0])ps[0]=px(ps[0]);return ps.join(' ')}}).join(', ')}}else{{v=px(v)}}}}return _sS.call(this,n,v)}};
document.addEventListener('click',function(e){{var t=e.target;while(t&&t.tagName!=='A')t=t.parentElement;if(t&&t.href&&!t.href.startsWith('javascript:')&&!t.href.startsWith('#')){{e.preventDefault();e.stopPropagation();var h=t.href;if(h.includes('localhost:9060')){{var m=h.match(/proxy\?url=(.+)$/);if(m)h=decodeURIComponent(m[1])}}window.parent.postMessage({{type:'navigate',url:h}},'*')}}}},true);
document.addEventListener('submit',function(e){{var f=e.target;if(f.tagName==='FORM'){{e.preventDefault();var fd=new FormData(f);var qs=new URLSearchParams(fd).toString();var u=f.action||O;if(u.includes('localhost:9060')){{var m=u.match(/proxy\?url=(.+)$/);if(m)u=decodeURIComponent(m[1])}}if(f.method&&f.method.toLowerCase()==='post'){{fetch(P+encodeURIComponent(u),{{method:'POST',body:fd}}).then(function(r){{return r.text()}}).then(function(h){{document.open();document.write(h);document.close()}}).catch(function(){{}})}}else{{if(qs)u+=(u.includes('?')?'&':'?')+qs;window.parent.postMessage({{type:'navigate',url:u}},'*')}}}}}},true);
var obs=new MutationObserver(function(ms){{ms.forEach(function(m){{m.addedNodes.forEach(function(n){{if(n.nodeType===1){{['src','href','poster','data-src','ping','formaction'].forEach(function(a){{var v=n.getAttribute&&n.getAttribute(a);if(v&&!v.includes('localhost:9060')&&(v.startsWith('http')||v.startsWith('//'))){{n.setAttribute(a,px(v))}}}});if(n.querySelectorAll){{n.querySelectorAll('[src],[href],[poster],[data-src],[ping],[formaction]').forEach(function(el){{['src','href','poster','data-src','ping','formaction'].forEach(function(a){{var v=el.getAttribute(a);if(v&&!v.includes('localhost:9060')&&(v.startsWith('http')||v.startsWith('//'))){{el.setAttribute(a,px(v))}}}})}})}}}}}})}}}});
obs.observe(document.documentElement,{{childList:true,subtree:true}});
}})();
</script>"#,
        proxy, origin
    );

    if let Some(pos) = html.find("<!") {
        if html[pos..].to_lowercase().starts_with("<!doctype") {
            if let Some(end) = html[pos..].find('>') {
                return format!(
                    "{}{}{}",
                    &html[..pos + end + 1],
                    script,
                    &html[pos + end + 1..]
                );
            }
        }
    }
    format!("{}{}", script, html)
}
