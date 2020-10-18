
var StrongColor = "rgba(241,81,85,1)";

var Base = `h1, h2, h3, h4, h5, p, li, strong, ::slotted(*) {
    font-family:"Quicksand_Book";
}`

class Menu extends HTMLElement {
    constructor() {
        super();
        this._pages = [{
                code : "index",
                name : "Home",
                link : "/",
            }, {
                code : 'diplomatie',
                name : "Diplomatie",
                link : "/pages/diplomatie.html",
            }, {
                code : "metier",
                name : "Métiers",
                link : "/pages/metiers.html",
            }, {
                code : "talent",
                name : "Talents",
                link : "/pages/talents.html",
            }
        ]

        this.attachShadow({ mode: 'open' });
        this.shadowRoot.innerHTML = `
        <style>
            ${Base}
                @import url(https://fonts.googleapis.com/css?family=Raleway:400,500,800);
                .menu {
                  font-family: 'Raleway', Arial, sans-serif;
                  text-align: center;
                  text-transform: uppercase;
                  font-weight: 500;
                }
                .menu * {
                  box-sizing: border-box;
                }
                .menu li {
                  display: inline-block;
                  list-style: outside none none;
                  margin: 0 1.5em;
                  padding: 0;
                }

                nav{
                    position: absolute;
                    margin-top: 50px;
                    margin-left: 200px;
                }

                #logo{
                    float:left;
                    width:150px;
                    height:150px;
                }

                #header{
                    height:150px;
	                border-bottom:10px ridge ${StrongColor};
                }

            </style>
            <div id="header">
                <div id="logo">
                    <img src="/img/logo.jpg">
                </div>
                <nav>
                    <ul class="menu">
                    </ul>
                </nav>
            </div>
        `
    }

    connectedCallback() {
        var _selected = this.getAttribute('page');

        this._menu = this.shadowRoot.querySelector(".menu");
        const li = this._pages.map( page => {
            if ( page.code == _selected){
                return `<li><cac-link href="#" current="1" data-hover="${page.name}">${page.name}</cac-link></li>`
            }else{
               return  `<li><cac-link href="${page.link}" current="0" data-hover="${page.name}">${page.name}</cac-link></li>`
            }
        })

        this._menu.innerHTML = li.join('')

    }
}
customElements.define('cac-menu', Menu);

class Content extends HTMLElement {

   constructor() {
        super();
        this.attachShadow({ mode: 'open' });


    this.shadowRoot.innerHTML = `
    <style>
        ${Base}

        h1{
            color: ${StrongColor};
            margin-bottom: 50px;
            text-align: center;
        }

        main {
            color:white;
            background:linear-gradient( #1f203c, #26284e);
            padding:50px;
            border-bottom:2px solid ${StrongColor};
        }

        ::slotted(ul){
            list-style-type: none;
        }

        #footer{
            height:100px;
            background-color:#1f203c;
            position:relative;
            bottom:0;
            width:100%;
        }

        #footer>ul{
            list-style-type: none;
        }

        #footer li{
            text-align:center;
            float:left;
            width:100px;
        }

        #footer>p{
            color:white;
            float:right;
            margin-right:50px;
        }

    </style>
    <main>
        <h1></h1>
        <slot></slot>
    </main>
    <div id="footer">
    		<ul>
    			<li><cac-link href="https://discord.gg/zgkXV4W">Discord</cac-link></li>
    			<li><cac-link href="https://crowfall.com/fr-FR/guilds/search?name=arbre%20clair">Guild in game</cac-link></li>
    			<li><cac-link href="https://github.com/Aedius/arbre-clair">Github</cac-link></li>
    			<li><cac-link href="https://royogaming00.wixsite.com/lecrowmalin">Le Crow Malin</cac-link></li>
    		</ul>
    		<p>Tout droits r&eacute;serv&eacute;s à Aedius. Copyright&copy; 2020</p>
    </div>
    `;

    }

  connectedCallback() {

    var _title = this.getAttribute('title');

    var node = this.shadowRoot.querySelector("h1");

    node.innerHTML= _title;

  }

}
customElements.define('cac-content', Content);

class Link extends HTMLElement {
    static get observedAttributes() {
        return ['current'];
    }
    constructor() {
        super();
        this.attachShadow({ mode: 'open' });

                this.shadowRoot.innerHTML = `
                <style>
                    ${Base}
                    a {
                      padding: 0.5em 0;
                      color: rgba(255, 255, 255, 0.5);
                      position: relative;
                      letter-spacing: 1px;
                      text-decoration: none;
                      line-height:200%
                    }
                    a.current{
                      color: white;
                    }

                    a:before,
                    a:after {
                      position: absolute;
                      -webkit-transition: all 0.35s ease;
                      transition: all 0.35s ease;
                    }
                    a:before {
                      bottom: 0;
                      display: block;
                      height: 3px;
                      width: 0%;
                      content: "";
                      background-color: ${StrongColor};
                    }
                    a:after {
                      left: 0;
                      top: 0;
                      padding: 0.5em 0;
                      position: absolute;
                      content: attr(data-hover);
                      color: #ffffff;
                      white-space: nowrap;
                      max-width: 0%;
                      overflow: hidden;
                    }
                    a:hover:before,
                    a.current:before {
                      opacity: 1;
                      width: 100%;
                    }
                    a:hover:after,
                    a.current:after {
                      max-width: 100%;
                    }

                </style>
                <a href="" class="">
                    <slot></slot>
                </a>
                `;

        this._node = this.shadowRoot.querySelector("a");
    }

      connectedCallback() {

        var _href = this.getAttribute('href');
        var _class = this.getAttribute('current') == 1 ? "current" : "";

        this._node.href=_href
        this._node.className = _class
      }
    attributeChangedCallback(name, oldValue, newValue) {

        var _class = newValue == 1 ? "current" : "";
        this._node.className = _class
    }
}
customElements.define('cac-link', Link);

class Banner extends HTMLElement {

    constructor() {
        super();
        this.attachShadow({ mode: 'open' });


    this.shadowRoot.innerHTML = `
        <style>
            ${Base}
            .banner {
                position:relative;
                background-size: cover;
                width: 100%;
                height: 300px;
                border-bottom:10px ridge ${StrongColor};
            }
            .banner img {
                display: none;
            }

            .sstitre_banner{
                color:#fff;
                font-size:0.6em;
            }

            #titre_banner{
                position:absolute;
                width:35%;
                padding:5px;
                margin-left:5%;
                margin-top:90px;
                background:rgba(0,0,0,0.35);
                border-radius: 10px;
                box-shadow: 1px 3px 3px #000;
            }

            #titre_banner>p{
                color:white;
                margin-left:5%;
                font-size:1.5em;
            }
        </style>
        <div id="container" class="banner" style="">
            <img src="" alt="" />
            <div id="titre_banner">
                <p id="titre_content">titre</p>
                <p><span  id="sstitre" class="sstitre_banner">sous titre</span></p>
            </div
        </div>
    `;

    }

  connectedCallback() {

    var _src = this.getAttribute('img');
    var _title = this.getAttribute('title');
    var _subtitle = this.getAttribute('subtitle');

    var nodeImg = this.shadowRoot.querySelector("img");
    nodeImg.src = _src
    nodeImg.alt = _title

    var nodeContainer = this.shadowRoot.querySelector("#container");
    nodeContainer.style = `background-image:url(${_src})`

    var nodeTitre = this.shadowRoot.querySelector("#titre_content");
    nodeTitre.innerHTML = _title

    var nodeSstitre = this.shadowRoot.querySelector("#sstitre");
    nodeSstitre.innerHTML = _subtitle
  }

}
customElements.define('cac-banner', Banner);


class Strong extends HTMLElement {

    constructor() {
        super();
        this.attachShadow({ mode: 'open' });

            this.shadowRoot.innerHTML = `
            <style>
                ${Base}
                strong{
                    color:${StrongColor};
                    font-size: 1.2em;
                }
            </style>
            <strong>
                <slot></slot>
            </strong>
            `;
    }

}
customElements.define('cac-strong', Strong);

class Li extends HTMLElement {

    constructor() {
        super();
        this.attachShadow({ mode: 'open' });

            this.shadowRoot.innerHTML = `
            <style>
                ${Base}

                li:before{
                    content: "\\2022";
                    margin-right: 10px;
                    color: ${StrongColor};
                }
            </style>
            <li>
                <slot></slot>
            </li>
            `;
    }

}
customElements.define('cac-li', Li);