"use strict";(self.webpackChunktuningplayground=self.webpackChunktuningplayground||[]).push([[806],{7065:(e,n,t)=>{Object.defineProperty(n,"__esModule",{value:!0}),n.requestMIDI=function(){navigator.requestMIDIAccess?navigator.requestMIDIAccess().then(a,c):alert("WebMIDI is not supported in this browser.")},n.stopMIDIFile=function(){d.forEach((e=>clearTimeout(e))),d=[]},n.playMIDIFile=function(e){new i.Midi(e).tracks.forEach((e=>{e.notes.forEach((e=>{const n=e.time*r.midiMultiplier-0,t=(e.time+e.duration)*r.midiMultiplier-0,i=e.velocity;1===i&&(e.velocity=127);const a=e.midi;d.push(setTimeout((()=>(0,o.noteOn)(a,i)),n)),d.push(setTimeout((()=>(0,o.noteOff)(a)),t))}))}))};const o=t(7806),i=t(3748),r=t(7206);function a(e){const n=e.inputs.values().next().value;n?n.onmidimessage=s:alert("No MIDI input devices found.")}function c(e){}function s(e){const[n,t,i]=e.data,r=144==(240&n);128==(240&n)&&(0,o.noteOff)(t),r&&(0,o.noteOn)(t,i)}let d=[]},5048:(e,n)=>{Object.defineProperty(n,"__esModule",{value:!0}),n.Tone=void 0,n.createTone=function(e,n,o,i,r){return new t(e,n,o,i,r)};class t{index;freq;cents;name;node;constructor(e,n,t,o,i){this.index=e,this.freq=n,this.cents=t,this.name=o,this.node=i}}n.Tone=t},9702:(e,n,t)=>{Object.defineProperty(n,"__esModule",{value:!0}),n.volumeValue=n.tranposeValue=n.output=n.volumeSlider=n.tuningSelect=n.playMarked=n.shareMarked=n.playButton=n.markedButtons=n.soundMethod=void 0,n.play=function(){(function(){g||(g=fetch("sample.mid").then((e=>e.arrayBuffer())).then((e=>(m=e,m))).catch((e=>{throw e})));return g})().then(c.playMIDIFile).catch(console.error)},n.DOMContentLoaded=function(){},n.handleTuningSelectChange=_,n.playingTonesChanged=v,n.logToDiv=k,n.keyActive=function(e,n){const t=document.querySelector(`div[data-note="${e}"]`);t&&(n?t.classList.add("key-active"):t.classList.remove("key-active"))},n.markKey=I,n.unmarkKey=T,n.markOrUnmarkKey=O,n.addEvents=function(e){const n=parseInt(e.getAttribute("data-note")),t=(n,t)=>{e.addEventListener(n,t)};e.addEventListener("mousedown",(e=>{e.ctrlKey?O(n):(0,a.noteOn)(n)})),t("mouseup",(()=>(0,a.noteOff)(n))),e.addEventListener("mouseenter",(e=>{e.ctrlKey||(0,a.noteOn)(n)})),t("mouseleave",(()=>(0,a.noteOff)(n))),t("touchstart",(()=>(0,a.noteOn)(n))),t("touchend",(()=>(0,a.noteOff)(n)))};const o=t(1635),i=o.__importStar(t(6584)),r=o.__importStar(t(3400)),a=t(7806),c=t(7065),s=document.getElementById("octaveSize"),d=document.getElementById("stepSize"),u=document.getElementById("fileInput");n.soundMethod=document.getElementById("soundMethod");const l=document.getElementById("logContainer"),f=d.parentElement;n.markedButtons=document.getElementById("markedButtons"),n.playButton=document.getElementById("playButton"),n.shareMarked=document.getElementById("shareMarked"),n.playMarked=document.getElementById("playMarked");const y=document.getElementById("stopButton");n.tuningSelect=document.getElementById("tuningSelect"),n.volumeSlider=document.getElementById("volumeSlider");const p=document.getElementById("transpose");let m;n.output=document.getElementById("output"),s.onchange=_,n.tuningSelect.onchange=_,d.onchange=_,u.onchange=function(e){return new Promise(((n,t)=>{const o=e.target.files;if(o&&o.length>0){const e=new FileReader;e.onload=e=>{m=e.target.result,g=Promise.resolve(m),n()},e.onerror=t,e.readAsArrayBuffer(o[0])}else t(new Error("No file selected"))}))},p.onchange=function(){n.tranposeValue=parseInt(p.value)},n.volumeSlider.onchange=function(){n.volumeValue=parseFloat(n.volumeSlider.value)},y.onclick=function(){(0,c.stopMIDIFile)()},n.playMarked.onclick=function(){a.markedKeys.forEach((e=>(0,a._noteOn)(e,void 0,!0)))},n.shareMarked.onclick=function(){b(a.markedKeys)()},n.tranposeValue=0,n.volumeValue=.25;let g=null;function _(){switch(n.tuningSelect.value){case"StepMethod":f.hidden=!1,d.readOnly=!1,s.readOnly=!1;break;case"EqualTemperament":f.hidden=!0,d.readOnly=!0,s.readOnly=!1;break;default:i.set_tuning_system(n.tuningSelect.value,parseInt(s.value),parseInt(d.value)),s.value=i.get_tuning_size().toString(),s.readOnly=!0,d.hidden=!0,d.readOnly=!0}(0,a.stopAllTones)()}function h(){n.output.style.width="300px",n.output.style.height="200px"}function v(){const e=Object.keys(a.playingTones).map(Number);if(0===e.length)return r.renderAbc("output",'X: 1\nL: 1/1\n|""[u]|'),void h();let n;const t=Object.values(a.playingTones).map((e=>e.name)).join(" ");if("12"===s.value){const e=i.convert_notes(t.split(" "));n=i.get_chord_name(),r.renderAbc("output",e),h()}k(`${t} | ${n}`,e)}function b(e){const n=w(e),t=`${window.location.origin+window.location.pathname}#${n}`;return function(){navigator.clipboard.writeText(t).catch(console.error)}}function w(e){return e.join(",")}function k(e,n){const t=document.createElement("p");t.textContent=e;const o=document.createElement("button");o.textContent="Share",o.onclick=b(n),o.style.marginRight="10px",t.style.marginLeft="10px";const i=document.createElement("div");i.style.display="flex",i.style.justifyContent="left",i.style.alignItems="center",i.appendChild(o),i.appendChild(t),l.insertBefore(i,l.firstChild)}function I(e){if(a.markedKeys.includes(e))return;a.markedKeys.push(e);const t=document.querySelector(`div[data-note="${e}"]`);t&&t.classList.add("key-marked"),n.markedButtons.style.display="block"}function T(e){const t=a.markedKeys.indexOf(e);t>-1&&a.markedKeys.splice(t,1);const o=document.querySelector(`div[data-note="${e}"]`);o&&o.classList.remove("key-marked"),0===a.markedKeys.length&&(n.markedButtons.style.display="none")}function O(e){a.markedKeys.indexOf(e)>-1?T(e):I(e),a.markedKeys.sort(((e,n)=>e-n)),window.location.hash=w(a.markedKeys)}},7206:(e,n)=>{Object.defineProperty(n,"__esModule",{value:!0}),n.midiMultiplier=void 0,n.midiMultiplier=1e3},5949:(e,n,t)=>{Object.defineProperty(n,"__esModule",{value:!0}),n.visibilityChange=function(){document.hidden&&(0,r.stopAllTones)()},n.onload=function(){const e=window.location.hash.substring(1);if(e){const n=e.split(",");a.markedButtons.style.display="flex",n.forEach((e=>{const n=parseInt(e);(0,a.markKey)(n)}))}else a.markedButtons.style.display="none"},n.keydown=function(e){if(!document.hasFocus())return;if(e.repeat)return;if(e.code in i.heldKeys)return;if("BODY"===document.activeElement?.tagName){const n=o.from_keymap(e.code);if(-1===n)return;(0,i.noteOn)(n),i.heldKeys[e.code]=!0}},n.keyup=function(e){const n=o.from_keymap(e.code);if(-1===n)return;(0,i.noteOff)(n),delete i.heldKeys[e.code]};const o=t(1635).__importStar(t(6584)),i=t(7806),r=t(7806),a=t(9702)},7806:(e,n,t)=>{Object.defineProperty(n,"__esModule",{value:!0}),n.markedKeys=n.heldKeys=n.playingTones=void 0,n.stopAllTones=s,n.noteOn=function(e,n,t){d(e,n,t),(0,c.playingTonesChanged)()},n._noteOn=d,n.noteOff=u;const o=t(1635).__importStar(t(6584)),i=t(5048),r=t(7065),a=t(5949),c=t(9702);function s(){Object.keys(n.playingTones).forEach((e=>{const t=parseInt(e);n.playingTones[t].node.stop(),delete n.playingTones[t],(0,c.keyActive)(t,!1)})),(0,c.playingTonesChanged)()}function d(e,t,i){e+=c.tranposeValue;const r=o.get_tone(e),a=Math.pow(c.volumeValue,2);switch(c.soundMethod.value){case"native":(async function(e,t){const o=await f(),i=o.createOscillator(),r=o.createGain();r.gain.value=t,r.connect(o.destination),i.type="square",i.frequency.setValueAtTime(e.freq,o.currentTime),i.connect(r),i.start(),e.node=i,e.index in n.playingTones&&n.playingTones[e.index].node.stop();n.playingTones[e.index]=e,(0,c.playingTonesChanged)()})(r,a).catch(console.error);break;case"sample":(async function(e,t,o){const i=await f(),r=i.createBufferSource();r.buffer=await(y?Promise.resolve(y):fetch("a1.wav").then((e=>e.arrayBuffer())).then((e=>f().then((n=>n.decodeAudioData(e))))).then((e=>(y=e,y))));const a=i.createGain();a.gain.value=t,r.connect(a),a.connect(i.destination),r.playbackRate.value=e.freq/220,r.start(),e.node=r,n.playingTones[e.index]=e,(0,c.playingTonesChanged)(),o&&(r.onended=()=>{u(e.index)})})(r,a,i).catch(console.error)}(0,c.keyActive)(e,!0)}function u(e){if((e+=c.tranposeValue)in n.playingTones){if("native"===c.soundMethod.value)n.playingTones[e].node.stop();delete n.playingTones[e],(0,c.playingTonesChanged)(),(0,c.keyActive)(e,!1)}}document.addEventListener("DOMContentLoaded",c.DOMContentLoaded),document.addEventListener("visibilitychange",a.visibilityChange),window.addEventListener("blur",s),window.addEventListener("hashchange",a.onload),window.createTone=i.createTone,o.default().then((()=>{(0,r.requestMIDI)(),c.playButton.onclick=c.play,document.addEventListener("keydown",a.keydown),document.addEventListener("keyup",a.keyup),document.querySelectorAll(".white-key, .black-key").forEach((e=>{(0,c.addEvents)(e)})),(0,a.onload)(),(0,c.playingTonesChanged)()})).catch(console.error),n.playingTones=[],n.heldKeys={},n.markedKeys=[];let l=null;function f(){return new Promise(((e,n)=>{try{l||(l=new window.AudioContext),e(l)}catch(e){n(e)}}))}let y=null},4747:(e,n,t)=>{e.exports=t.p+"4cee9edc518db67536e9.wasm"},6584:(e,n,t)=>{let o;t.r(n),t.d(n,{convert_notes:()=>T,default:()=>A,from_keymap:()=>k,get_chord_name:()=>I,get_tone:()=>b,get_tuning_size:()=>w,initSync:()=>x,main:()=>h,set_tuning_system:()=>O});const i=new Array(128).fill(void 0);function r(e){return i[e]}i.push(void 0,null,!0,!1);let a=0,c=null;function s(){return null!==c&&0!==c.byteLength||(c=new Uint8Array(o.memory.buffer)),c}const d="undefined"!=typeof TextEncoder?new TextEncoder("utf-8"):{encode:()=>{throw Error("TextEncoder not available")}},u="function"==typeof d.encodeInto?function(e,n){return d.encodeInto(e,n)}:function(e,n){const t=d.encode(e);return n.set(t),{read:e.length,written:t.length}};function l(e,n,t){if(void 0===t){const t=d.encode(e),o=n(t.length,1)>>>0;return s().subarray(o,o+t.length).set(t),a=t.length,o}let o=e.length,i=n(o,1)>>>0;const r=s();let c=0;for(;c<o;c++){const n=e.charCodeAt(c);if(n>127)break;r[i+c]=n}if(c!==o){0!==c&&(e=e.slice(c)),i=t(i,o,o=c+3*e.length,1)>>>0;const n=s().subarray(i+c,i+o);c+=u(e,n).written,i=t(i,o,c,1)>>>0}return a=c,i}let f=null;function y(){return(null===f||!0===f.buffer.detached||void 0===f.buffer.detached&&f.buffer!==o.memory.buffer)&&(f=new DataView(o.memory.buffer)),f}let p=i.length;function m(e){const n=r(e);return function(e){e<132||(i[e]=p,p=e)}(e),n}const g="undefined"!=typeof TextDecoder?new TextDecoder("utf-8",{ignoreBOM:!0,fatal:!0}):{decode:()=>{throw Error("TextDecoder not available")}};function _(e,n){return e>>>=0,g.decode(s().subarray(e,e+n))}function h(){o.main()}function v(e){p===i.length&&i.push(i.length+1);const n=p;return p=i[n],i[n]=e,n}function b(e){return m(o.get_tone(e))}function w(){return o.get_tuning_size()>>>0}function k(e){const n=l(e,o.__wbindgen_export_0,o.__wbindgen_export_1),t=a;return o.from_keymap(n,t)}function I(){let e,n;try{const r=o.__wbindgen_add_to_stack_pointer(-16);o.get_chord_name(r);var t=y().getInt32(r+0,!0),i=y().getInt32(r+4,!0);return e=t,n=i,_(t,i)}finally{o.__wbindgen_add_to_stack_pointer(16),o.__wbindgen_export_2(e,n,1)}}function T(e){let n,t;try{const c=o.__wbindgen_add_to_stack_pointer(-16),s=function(e,n){const t=n(4*e.length,4)>>>0,o=y();for(let n=0;n<e.length;n++)o.setUint32(t+4*n,v(e[n]),!0);return a=e.length,t}(e,o.__wbindgen_export_0),d=a;o.convert_notes(c,s,d);var i=y().getInt32(c+0,!0),r=y().getInt32(c+4,!0);return n=i,t=r,_(i,r)}finally{o.__wbindgen_add_to_stack_pointer(16),o.__wbindgen_export_2(n,t,1)}}function O(e,n,t){const i=l(e,o.__wbindgen_export_0,o.__wbindgen_export_1),r=a;o.set_tuning_system(i,r,n,t)}function E(){const e={wbg:{}};return e.wbg.__wbg_createTone_ee958f0cdcf79124=function(e,n,t,i,r,a){let c,s;try{c=i,s=r;return v(createTone(e>>>0,n,t,_(i,r),m(a)))}finally{o.__wbindgen_export_2(c,s,1)}},e.wbg.__wbindgen_string_get=function(e,n){const t=r(n),i="string"==typeof t?t:void 0;var c=null==i?0:l(i,o.__wbindgen_export_0,o.__wbindgen_export_1),s=a;y().setInt32(e+4,s,!0),y().setInt32(e+0,c,!0)},e.wbg.__wbindgen_object_drop_ref=function(e){m(e)},e.wbg.__wbindgen_throw=function(e,n){throw new Error(_(e,n))},e}function M(e,n){return o=e.exports,B.__wbindgen_wasm_module=n,f=null,c=null,o.__wbindgen_start(),o}function x(e){if(void 0!==o)return o;void 0!==e&&Object.getPrototypeOf(e)===Object.prototype&&({module:e}=e);const n=E();e instanceof WebAssembly.Module||(e=new WebAssembly.Module(e));return M(new WebAssembly.Instance(e,n),e)}async function B(e){if(void 0!==o)return o;void 0!==e&&Object.getPrototypeOf(e)===Object.prototype&&({module_or_path:e}=e),void 0===e&&(e=new URL(t(4747),t.b));const n=E();("string"==typeof e||"function"==typeof Request&&e instanceof Request||"function"==typeof URL&&e instanceof URL)&&(e=fetch(e));const{instance:i,module:r}=await async function(e,n){if("function"==typeof Response&&e instanceof Response){if("function"==typeof WebAssembly.instantiateStreaming)try{return await WebAssembly.instantiateStreaming(e,n)}catch(n){if("application/wasm"==e.headers.get("Content-Type"))throw n}const t=await e.arrayBuffer();return await WebAssembly.instantiate(t,n)}{const t=await WebAssembly.instantiate(e,n);return t instanceof WebAssembly.Instance?{instance:t,module:e}:t}}(await e,n);return M(i,r)}"undefined"!=typeof TextDecoder&&g.decode();const A=B}}]);