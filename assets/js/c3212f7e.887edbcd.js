"use strict";(self.webpackChunkribir_website=self.webpackChunkribir_website||[]).push([[797],{3905:(e,t,i)=>{i.d(t,{Zo:()=>p,kt:()=>h});var n=i(7294);function r(e,t,i){return t in e?Object.defineProperty(e,t,{value:i,enumerable:!0,configurable:!0,writable:!0}):e[t]=i,e}function a(e,t){var i=Object.keys(e);if(Object.getOwnPropertySymbols){var n=Object.getOwnPropertySymbols(e);t&&(n=n.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),i.push.apply(i,n)}return i}function o(e){for(var t=1;t<arguments.length;t++){var i=null!=arguments[t]?arguments[t]:{};t%2?a(Object(i),!0).forEach((function(t){r(e,t,i[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(i)):a(Object(i)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(i,t))}))}return e}function d(e,t){if(null==e)return{};var i,n,r=function(e,t){if(null==e)return{};var i,n,r={},a=Object.keys(e);for(n=0;n<a.length;n++)i=a[n],t.indexOf(i)>=0||(r[i]=e[i]);return r}(e,t);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);for(n=0;n<a.length;n++)i=a[n],t.indexOf(i)>=0||Object.prototype.propertyIsEnumerable.call(e,i)&&(r[i]=e[i])}return r}var l=n.createContext({}),s=function(e){var t=n.useContext(l),i=t;return e&&(i="function"==typeof e?e(t):o(o({},t),e)),i},p=function(e){var t=s(e.components);return n.createElement(l.Provider,{value:t},e.children)},u="mdxType",c={inlineCode:"code",wrapper:function(e){var t=e.children;return n.createElement(n.Fragment,{},t)}},m=n.forwardRef((function(e,t){var i=e.components,r=e.mdxType,a=e.originalType,l=e.parentName,p=d(e,["components","mdxType","originalType","parentName"]),u=s(i),m=r,h=u["".concat(l,".").concat(m)]||u[m]||c[m]||a;return i?n.createElement(h,o(o({ref:t},p),{},{components:i})):n.createElement(h,o({ref:t},p))}));function h(e,t){var i=arguments,r=t&&t.mdxType;if("string"==typeof e||r){var a=i.length,o=new Array(a);o[0]=m;var d={};for(var l in t)hasOwnProperty.call(t,l)&&(d[l]=t[l]);d.originalType=e,d[u]="string"==typeof e?e:r,o[1]=d;for(var s=2;s<a;s++)o[s]=i[s];return n.createElement.apply(null,o)}return n.createElement.apply(null,i)}m.displayName="MDXCreateElement"},2337:(e,t,i)=>{i.r(t),i.d(t,{assets:()=>l,contentTitle:()=>o,default:()=>u,frontMatter:()=>a,metadata:()=>d,toc:()=>s});var n=i(7462),r=(i(7294),i(3905));const a={},o="Core principles",d={unversionedId:"advanced_topics/framework",id:"advanced_topics/framework",title:"Core principles",description:"Phase",source:"@site/../docs/advanced_topics/framework.md",sourceDirName:"advanced_topics",slug:"/advanced_topics/framework",permalink:"/docs/advanced_topics/framework",draft:!1,editUrl:"https://github.com/RibirX/Ribir/tree/master/website/../docs/advanced_topics/framework.md",tags:[],version:"current",lastUpdatedBy:"sologeek",lastUpdatedAt:1676366933,formattedLastUpdatedAt:"Feb 14, 2023",frontMatter:{},sidebar:"tutorialSidebar",previous:{title:"Architecture",permalink:"/docs/advanced_topics/architecture"},next:{title:"Custom widget declare in macro.",permalink:"/docs/custom_widget_declare_in_macro"}},l={},s=[{value:"Phase",id:"phase",level:2},{value:"Declare &amp; Reactive programming mode",id:"declare--reactive-programming-mode",level:2},{value:"Rebuild Widget Subtree Diff Algorithm",id:"rebuild-widget-subtree-diff-algorithm",level:2},{value:"Key",id:"key",level:3},{value:"Compose prefer",id:"compose-prefer",level:2},{value:"Widget Attribute",id:"widget-attribute",level:3},{value:"RenderObject &amp; RenderTree",id:"renderobject--rendertree",level:2},{value:"Stateless and Stateful",id:"stateless-and-stateful",level:2},{value:"Layout",id:"layout",level:3},{value:"Children Relationship",id:"children-relationship",level:2},{value:"avoid to rebuild widget ?",id:"avoid-to-rebuild-widget-",level:2}],p={toc:s};function u(e){let{components:t,...i}=e;return(0,r.kt)("wrapper",(0,n.Z)({},p,i,{components:t,mdxType:"MDXLayout"}),(0,r.kt)("h1",{id:"core-principles"},"Core principles"),(0,r.kt)("h2",{id:"phase"},"Phase"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre"},"                    Main thread                        |             Parallelism   \n                        .                              |                   \n  Build Phase         --.--\x3e  Render Ready Phase    ---|--\x3e    Layout    --|--\x3e Paint\n                        .                              |                   |\n")),(0,r.kt)("p",null,"inflate the widget tree     .   construct render tree and  |                   |\nor rebuild subtree        . update render tree data from |                   |\n.         widget tree          |                   |\n<------------------- May back build phase again <-----|"),(0,r.kt)("h2",{id:"declare--reactive-programming-mode"},"Declare & Reactive programming mode"),(0,r.kt)("p",null,(0,r.kt)("inlineCode",{parentName:"p"},"Ribir")," builds widget tree with your ui by declare data, meanwhile it creates render object tree to layout and paint.\nWhen the data that the widget depends on changes, widget tree will make a update, and render objects correspond updated widgets to update also."),(0,r.kt)("p",null,"When a widget as root node run in ",(0,r.kt)("inlineCode",{parentName:"p"},"Application"),", it will be inflated into widget tree by framework. Every leaf in the widget tree is a rendered widget. Sometimes, only has tree updated is not enough, it's possible that ",(0,r.kt)("inlineCode",{parentName:"p"},"CombinationWidget")," builds a complete full differently ",(0,r.kt)("inlineCode",{parentName:"p"},"Widget"),".  So if a ",(0,r.kt)("inlineCode",{parentName:"p"},"CombinationWidget")," is changed, it need rebuild and maybe reconstruct full subtree. Framework try to rebuild the widget tree and the render tree as mini as possible."),(0,r.kt)("h2",{id:"rebuild-widget-subtree-diff-algorithm"},"Rebuild Widget Subtree Diff Algorithm"),(0,r.kt)("p",null,"Widget doesn't update or rebuild subtree immediately when its state changed. It's just mark this widget need to rebuild and wait until the widget tree rebuild. "),(0,r.kt)("p",null,"The widget tree update from top to bottom. If a bottom widget removed because its ancestor rebuild, its update or rebuild auto be canceled. Even if ",(0,r.kt)("inlineCode",{parentName:"p"},"CombinationWidget")," require rebuild, itself must be rebuild, but that not mean ",(0,r.kt)("inlineCode",{parentName:"p"},"Ribir")," will reconstruct the total subtree, the ",(0,r.kt)("inlineCode",{parentName:"p"},"Key"),"may help us to reduce many cost in some case."),(0,r.kt)("h3",{id:"key"},"Key"),(0,r.kt)("p",null,(0,r.kt)("inlineCode",{parentName:"p"},"Key")," helps ",(0,r.kt)("inlineCode",{parentName:"p"},"Ribir")," to track what widgets add, remove and changed. So ",(0,r.kt)("inlineCode",{parentName:"p"},"Ribir")," can modify the widget tree and the render tree minimally. A ",(0,r.kt)("inlineCode",{parentName:"p"},"Key")," should unique for each widget under the same father."),(0,r.kt)("p",null,"The widget tree rebuilds base on widget diff. Work like below:"),(0,r.kt)("p",null,"a. build widget from ",(0,r.kt)("inlineCode",{parentName:"p"},"CombinationWidget"),".\nb. if the ",(0,r.kt)("inlineCode",{parentName:"p"},"key")," of widget is equal to the last time build widget in the widget tree ?"),(0,r.kt)("ol",null,(0,r.kt)("li",{parentName:"ol"},"use new widget replace before sub tree in widget and mark this widget dirty."),(0,r.kt)("li",{parentName:"ol"},"if this widget is ",(0,r.kt)("inlineCode",{parentName:"li"},"CombinationWidget"),", use new widget recursive step a."),(0,r.kt)("li",{parentName:"ol"},"else, if this widget is render widget and has children.")),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre"},"* pluck all children from widget tree.\n* process new children one by one\n  - if a old child can be found by new child's key in plucked children.\n    * insert old child back.\n    * recursive step 1.\n  - else add new widget in the widget tree, and recursive step c.\n  - destroy the remaining plucked child and subtree, correspond render tree destroy too.\n")),(0,r.kt)("ol",{start:4},(0,r.kt)("li",{parentName:"ol"},"else done.\nc. else, inflate the new widget and use the new widget subtree instead of the old in widget tree, reconstruct render subtree correspond to this widget subtree.\nd. done, the subtree from this widget is rebuild finished.")),(0,r.kt)("h2",{id:"compose-prefer"},"Compose prefer"),(0,r.kt)("p",null,"Unlike many classic GUI framework, ",(0,r.kt)("inlineCode",{parentName:"p"},"Ribir")," doesn't build on inherit mode. Because Rust not support inherit, ",(0,r.kt)("inlineCode",{parentName:"p"},"Ribir")," built base on composition. For example, If you want give a ",(0,r.kt)("inlineCode",{parentName:"p"},"Button")," widget opacity, it doesn't have a field named ",(0,r.kt)("inlineCode",{parentName:"p"},"opacity")," give you to set the opacity, you should use a ",(0,r.kt)("inlineCode",{parentName:"p"},"Opacity")," widget to do it, like:"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},'Opacity {\n  opacity: 0.5,\n  Button! { text: "Click Me!"}\n}\n')),(0,r.kt)("h3",{id:"widget-attribute"},"Widget Attribute"),(0,r.kt)("h2",{id:"renderobject--rendertree"},"RenderObject & RenderTree"),(0,r.kt)("p",null,"The widget tree corresponds to the user interface, and ",(0,r.kt)("inlineCode",{parentName:"p"},"RenderTree")," is created by RenderWidget, it do the actually layout and paint. In this vision, widget is build cheap, and RenderObject is more expensive than widget."),(0,r.kt)("h2",{id:"stateless-and-stateful"},"Stateless and Stateful"),(0,r.kt)("p",null,"As default, every widget is stateless, just present like what you declare and no interactive. But in real world we often need change widget to another state to respond to user actions, IO request and so on. A way to support it is rebuild the whole widget tree and do a tree diff to update the minimal render tree. But we provide another way to do it, widget can across ",(0,r.kt)("inlineCode",{parentName:"p"},"into_stateful")," convert to a stateful widget, which can be used to reference the widget and modify the states of the widget."),(0,r.kt)("h3",{id:"layout"},"Layout"),(0,r.kt)("p",null,(0,r.kt)("inlineCode",{parentName:"p"},"Ribir")," performs a layout per frame, and the layout algorithm works in a single pass. It's a recursive layout from parent down to children. "),(0,r.kt)("p",null,"There is some important point to help understand how to write a layout:"),(0,r.kt)("ol",null,(0,r.kt)("li",{parentName:"ol"},"RenderObject not directly hold its children, and have no way to directly access them."),(0,r.kt)("li",{parentName:"ol"},(0,r.kt)("inlineCode",{parentName:"li"},"RenderTree")," store the ",(0,r.kt)("inlineCode",{parentName:"li"},"RenderObject"),"'s layout result, so ",(0,r.kt)("inlineCode",{parentName:"li"},"RenderObject")," only need to provide its layout algorithm in ",(0,r.kt)("inlineCode",{parentName:"li"},"perform_layout")," method.  The ",(0,r.kt)("inlineCode",{parentName:"li"},"perform_layout")," have two input, a ",(0,r.kt)("inlineCode",{parentName:"li"},"BoxClamp")," that limit the min and max size it can, a ",(0,r.kt)("inlineCode",{parentName:"li"},"RenderCtx")," provide the ability to call the ",(0,r.kt)("inlineCode",{parentName:"li"},"perform_layout")," of its children, and it a way to know the size the child need."),(0,r.kt)("li",{parentName:"ol"},(0,r.kt)("inlineCode",{parentName:"li"},"RenderObject::perform_layout")," responsible for calling every children's perform_layout across the ",(0,r.kt)("inlineCode",{parentName:"li"},"RenderCtx"),"\u3002"),(0,r.kt)("li",{parentName:"ol"},"The ",(0,r.kt)("inlineCode",{parentName:"li"},"BoxClamp")," it always gave by parent.")),(0,r.kt)("p",null,(0,r.kt)("inlineCode",{parentName:"p"},"only_sized_by_parent")," method can help framework know if the ",(0,r.kt)("inlineCode",{parentName:"p"},"RenderObject")," is the only input to detect the size, and children size not affect its size."),(0,r.kt)("h2",{id:"children-relationship"},"Children Relationship"),(0,r.kt)("p",null,"In ",(0,r.kt)("inlineCode",{parentName:"p"},"Ribir"),", normal render object not hold the children, but we can use layout widget to build a parent & children relationship."),(0,r.kt)("h2",{id:"avoid-to-rebuild-widget-"},"avoid to rebuild widget ?"))}u.isMDXComponent=!0}}]);