

class Menu extends HTMLElement {
    constructor() {
        super();
        this._pages = [{
                code : "index",
                name : "Homepage",
                link : "/",
            }, {
                code : 'diplomatie',
                name : "Diplomatie",
                link : "/pages/diplomatie.html",
            }, {
                code : "cuisine",
                name : "Cuisine",
                link : "/pages/cuisine.html",
            }
        ]

        this._link = [{
                name : "Discord",
                link : "https://discord.gg/zgkXV4W",
            }, {
                name : "Guild in game",
                link : "https://crowfall.com/fr-FR/guilds/search?name=arbre%20clair",
            }, {
                name : "Github",
                link : "https://github.com/Aedius/arbre-clair",
            }
        ]


        this.attachShadow({ mode: 'open' });
        this.shadowRoot.innerHTML = `
            <nav>
                <h4>Pages</h4>
                <ul class="menu">
                </ul>
                <h4>Liens</h4>
                <ul class="link">
                </ul>
            </nav>
        `
    }

    connectedCallback() {
        var _selected = this.getAttribute('page');

        this._menu = this.shadowRoot.querySelector(".menu");
        const li = this._pages.map( page => {
            if ( page.code == _selected){
                return `<li>${page.name}</li>`
            }else{
               return  `<li><a href="${page.link}">${page.name}</a></li>`
            }
        })

        this._menu.innerHTML = li.join('')

        this._menuLink = this.shadowRoot.querySelector(".link");

        const lil = this._link.map( page => {
               return  `<li><a href="${page.link}">${page.name}</a></li>`
            }
        )

        this._menuLink.innerHTML = lil.join('')
    }
}
customElements.define('cac-menu', Menu);

class Content extends HTMLElement {

    constructor() {
        super();
        this.attachShadow({ mode: 'open' });
    }

  connectedCallback() {

        var _title = this.getAttribute('title');

    this.shadowRoot.innerHTML = `
    <main>
        <h1>${_title}</h1>
        <slot></slot>
    </main>
    `;
  }

}
customElements.define('cac-content', Content);


class Header extends HTMLElement {

    constructor() {
        super();
        this.attachShadow({ mode: 'open' });
    }

  connectedCallback() {

    var _title = this.getAttribute('title');

    var _width = screen.availWidth / 5
    var _height = screen.availHeight / 10

    this.shadowRoot.innerHTML = `
    <style>
        img {
            height: ${_height}px;
        }
        header {
            height: ${_height}px;
            text-align: center;
            position: relative;
        }
        span {
            margin: 0;
            position: absolute;
            top: 50%;
            left: 50%;
            font-size:3em;
            transform: translate(-50%, -50%);
            font-family: 'greennature', 'Georgia', serif;
        }
        a {
            position: absolute;
            top: 0;
            left: 0;
        }
    </style>
    <header>
        <a href="/"><img src="/img/logo.png"></a>
        <span>[CAC] Communauté de l'Arbre Clair</div>
    </header>
    `;
  }

}
customElements.define('cac-header', Header);