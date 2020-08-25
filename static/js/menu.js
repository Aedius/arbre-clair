class Menu extends HTMLElement {
    constructor() {
        super();
        this._pages = ["index", "alliance" ]

        this._pagesData = {
            index : {
                "name" : "Homepage",
                "link" : "/",
            },
            alliance : {
                "name" : "Diplomatie",
                "link" : "/pages/alliance.html",
            }
        }

        this.attachShadow({ mode: 'open' });
        this.shadowRoot.innerHTML = `
            <nav>
                <h2>Menu</h2>
                <ul class="menu">
                </ul>
            </nav>
        `
    }

    connectedCallback() {
        var _selected = this.getAttribute('page');

        this._menu = this.shadowRoot.querySelector(".menu");
        console.log( this._selected)
        const li = this._pages.map( page => {

            const data = this._pagesData[page]

            console.log( data)

            if ( page == _selected){
                return `<li>${data.name}</li>`
            }else{
               return  `<li><a href="${data.link}">${data.name}</a></li>`
            }
        })


        this._menu.innerHTML = li.join('')
    }
}
customElements.define('cac-menu', Menu);