class RecipeContainer extends HTMLElement {

    constructor() {
        super();

        this._craft = [
            {
                code : "Alchemy",
                name : "Alchemy",
            }, {
                code : "Cooking",
                name : "Cooking",
            }, {
                code : 'Jewelry',
                name : "Jewelry",
            }, {
                code : 'StoneMasonry',
                name : "Stone masonry",
            }
        ]

        this._data=null

        this.attachShadow({ mode: 'open' });

        const li = this._craft.map( craft => {
            return  `<li><cac-link  current="0" href="/pages/metiers.html#${craft.code}" id="${craft.code}" data-hover="${craft.name}">${craft.name}</cac-link></li>`
        })

        this.shadowRoot.innerHTML = `
            <style>
                ul{
                    list-style-type: none;
                }
                li {
                     display: inline-block;
                     margin: 0 0.5em;
                     font-size: 1.8em;
                }
            </style>
            <nav>
                <ul>
                    ${li.join('')}
                </ul>
            </nav>
            <cac-recipe-list kind=""></cac-recipe-list>
            <cac-recipe kind=""></cac-recipe>
        `

        this._recipe_list = this.shadowRoot.querySelector("cac-recipe-list")
        this._recipe = this.shadowRoot.querySelector("cac-recipe")
    }

    connectedCallback() {
        this._draw()
        var those = this;
        window.onhashchange = function() {
            those._draw()
        }

    }

    _draw(){
        var _hash = window.location.hash
        var _selected = ""
        var _recipe = ""
        var _content= ""
        if(_hash){
            _hash = _hash.substring(1);
            var data = _hash.split("/")
            _selected=data[0];
            if(data.length>1){
                _recipe=data[1];
            }
        }

         var linkList = this.shadowRoot.querySelectorAll("nav cac-link");

         for (let l of linkList){
            l.setAttribute("current", 0);
         }

        if( _selected == ""){
            this._recipe_list.setAttribute("kind", "")
            this._recipe.setAttribute("kind", "")
        }else{
            if (_recipe == ""){
                this._recipe_list.setAttribute("kind", _selected)
                this._recipe.setAttribute("kind", "")
            }else{
                this._recipe_list.setAttribute("kind", "")
                this._recipe.setAttribute("kind", _recipe)
            }

            var currentLink = this.shadowRoot.querySelector("#"+_selected)
            currentLink.setAttribute("current", 1)
        }


    }

}
customElements.define('cac-recipe-container', RecipeContainer);

class RecipeList extends HTMLElement {

    static get observedAttributes() {
        return ['kind'];
    }

    constructor() {
        super();

        this.attachShadow({ mode: 'open' });

        this.shadowRoot.innerHTML = `
            <style>
                ul{
                    list-style-type: none;
                }
            </style>
            <div>
                <ul>
                    Loading
                </ul>
            </div>
        `

        this._ul = this.shadowRoot.querySelector("ul")
    }


    connectedCallback() {
        let kind = this.getAttribute('kind');

        if (kind != ""){
            this._get_recipes(kind);
        }

    }

    attributeChangedCallback(name, oldValue, newValue) {

        if ( name == "kind" ) {
            if (newValue == "") {
                this._ul.style.display = "none";
            }else{
                this._ul.style.display = "";
                this._kind = newValue;
                this._get_recipes(newValue);
            }
        }
    }

    _get_recipes(kind){

        var those = this;

        fetch('/api/recipe-list/' + kind).then(function (response) {
            // The API call was successful!
            return response.json();
        }).then(function (data) {
            // This is the JSON from our response
            those._display(data)
        }).catch(function (err) {
            // There was an error
            console.warn('Something went wrong.', err);
        });
    }

    _display(_data){

        let li = _data.map( recipe => {
           return  `<li><cac-link  current="0" href="/pages/metiers.html#${this._kind}/${recipe.key}" >${recipe.name} :</cac-link>
                ${recipe.stat}</li>`
        }).join('')

        this._ul.innerHTML = li

    }

}
customElements.define('cac-recipe-list', RecipeList);


class Recipe extends HTMLElement {

    static get observedAttributes() {
        return ['kind'];
    }

    constructor() {
        super();

        this._data=null

        var lastNb =  parseInt(localStorage.getItem("recipeQuantity"));

        if(Number.isInteger(lastNb) && lastNb>1){
            this._nb=lastNb;
        }else{
            this._nb=1;
        }
        this.attachShadow({ mode: 'open' });

        this.shadowRoot.innerHTML = `
        <style>
           ${Base}

            h2{
                margin-left:40px;
                padding-bottom:10px;
                text-transform: uppercase;
                border-bottom:1mm ridge rgba(241,81,85,1);
                border-radius:10px;
            }

            #recette_cuisine{
                background-color:#fff;
                width:100%;
            }

            .nom_recette{
                width:35%;
                height:100%;
                float:left;
            }

            .ingredient{
                color:rgba(241,81,85,1);
                font-weight:bold;
            }

            .multiplier, .compo{
                color:rgba(241,81,85,1);
                font-weight:bold;
            }

            #step{
                background-color:#fff;
                color:#000;
                padding:20px;
                width:50%;
                border-left:2mm ridge rgba(241,81,85,1);
                border-right:2mm ridge rgba(241,81,85,1);
                border-radius:20px;
                margin-left:40%;
            }

            .step{
                width:80%;
                line-height:0.6em;
                border-bottom:1px solid #000;
                padding-bottom:10px;
                margin-left:10%;
                margin-right:10%;
            }

            .step:last-child{
                border-bottom:none;
            }

            .sous-titre{
                font-weight:bold;
            }

            .legende{
                margin-left:40px;
                font-size:0.8em;
            }
        </style>
        <div id="container">
            <div class="recette_cuisine">
                <div class="nom_recette">
                    <h2>
                    <cac-quantity nb="${this._nb}" callback=${this._quantity_change}></cac-quantity>
                    <span>recipe name</span></h2>
                <ul id="input">

                </ul>
                <p class="legende">*Passer sur les ingrédients pour connaitre la composition.</p>
            </div>
            <div id="step">
            </div>
        </div>
      `

        this.shadowRoot.addEventListener('quantityChange', this._quantity_change.bind(this));

        this._recipe_name = this.shadowRoot.querySelector(".recette_cuisine h2 span")
        this._input = this.shadowRoot.querySelector("#input")
        this._steps = this.shadowRoot.querySelector("#step")
        this._container = this.shadowRoot.querySelector("#container")
    }

    connectedCallback() {
        this._kind = this.getAttribute('kind');
        this._refresh();
    }

    attributeChangedCallback(name, oldValue, newValue) {

        if ( name == "kind" ) {
            if (newValue == "") {
                this._container.style.display = "none";
            }else{
                this._container.style.display = "";
                this._kind =newValue;

                this._refresh();
            }
        }
    }
    _quantity_change(e){
        localStorage.setItem('recipeQuantity', e.detail);
        this._nb =  e.detail;
        this._refresh();
    }

    _refresh(){

        if (this._kind == ""){
            return;
        }else{
        }

        var those = this

        fetch('/api/recipe/detail/' + this._kind + '/'+ this._nb).then(function (response) {
            // The API call was successful!
            return response.json();
        }).then(function (data) {
            // This is the JSON from our response
            those._data = data
            those._display()
        }).catch(function (err) {
            // There was an error
            console.warn('Something went wrong.', err);
        });
    }

    _display(){

            var liBase = this._data.base.map( resource => {
               return  `<li>${resource.quantity} x <cac-strong>${resource.base}</cac-strong></li>`
            }).join('')

            var liGroup = this._data.group.map( resource => {

                var groupComponent = resource.base_list.map( base => {
                    return `<p><cac-strong>${base}</cac-strong><p>`
                }).join('</p>ou<p>')

               return  `<li>${resource.quantity} x
                   <cac-desc name="${resource.group}" content="<p>${groupComponent}</p>"></cac-desc></li>`
                }).join('')

            var step = this._data.recipe.map( recipeGroup => {

                var group = recipeGroup.recipe_list.map( recipe => {

                    var total = recipe.quantity * recipe.recipe.output[1]

                    var input = recipe.recipe.input.map( input => {
                        return `<p>${input[1]} x <cac-strong>${input[0]}</cac-strong></p>`
                    }).join('')

                    return `
                        <p><span class="sous-titre">Profession : </span>${recipe.recipe.profession}</p>
                        <p><span class="sous-titre">Menu : </span>${recipe.recipe.menu}</p>
                        <div class="liste_ingredient">
                            Perform <span class="multiplier">${recipe.quantity}</spang> times :
                            <cac-desc name="${recipe.recipe.name}" content="${input}"></cac-desc>
                        </div>
                        <br/>
                        <p>
                        To get <span class="multiplier">${total}</span> x <span class="compo">${recipe.recipe.output[0]}</span>
                        </p>
                        <p>
                            ( already go )
                        </p>
                    `
                }).join('<br/>')

                return `

                <div class="step">
                    <h3>STEP ${recipeGroup.level}</h3>
                   ${group}
                </div>
                `
            }).join('')

        this._recipe_name.innerHTML = this._data.summary.name
        this._input.innerHTML = liBase + liGroup
        this._steps.innerHTML = step

    }
}
customElements.define('cac-recipe', Recipe);

class Description extends HTMLElement {

    constructor() {
        super();
        this.attachShadow({ mode: 'open' });
    }

  connectedCallback() {


    var _name = this.getAttribute('name');
    var _content = this.getAttribute('content');

    this.shadowRoot.innerHTML = `
    <style>
        ${Base}

        .compo a{
            text-decoration:none;
            color:rgba(241,81,85,1);
        }

        .compo a:hover,.compo a:focus{
            color:#09c;
            box-shadow:0 1px 0 rgba(255,255,255,.4);
        }

        .compo a span{
            position:absolute;
            margin-top:23px;
            margin-left:-35px;
            border:1px solid rgba(241,81,85,1);
            color:#000;
            background:linear-gradient(45deg,#e2e4fe,#c4c7ff);
            padding-left:15px;
            padding-right:15px;
            border-radius:5px;
            box-shadow:0 0 2px rgba(0,0,0,.5);
            transform:scale(0) rotate(-12deg);
            transition:all .25s;
            opacity:0;
        }

        .compo a:hover span,.compo a:focus span{
            transform:scale(1) rotate(0);
            opacity:1;
        }

    </style>
    <span class="compo">
        <a href="#"><cac-strong>${_name} *</cac-strong>
        <span>${_content}</span>
        </a>
    </span>
    `;
  }

}
customElements.define('cac-desc', Description);

class InputQuantity extends HTMLElement {

    constructor() {
        super();
        this.attachShadow({ mode: 'open' });
        this._nb=0;
    }


    _quantity_change(e){
        var nb = parseInt(e.target.value)

        console.log(nb);
        console.log(this._nb);

        if( nb == this._nb){
            return;
        }

        if( Number.isInteger(nb)){
            this.dispatchEvent(
                new CustomEvent("quantityChange", {
                    bubbles: true,
                    cancelable: false,
                    detail: nb,
                })
            )
        }
    }


    connectedCallback() {

        console.log("render quantity")

        this._nb = parseInt(this.getAttribute('nb'));
        var _callback = this.getAttribute('callback');

        this.shadowRoot.innerHTML = `
            <input value="${this._nb}" size="4" type="number" min="1" />
        `;

        this.shadowRoot.querySelector("input").addEventListener('change', this._quantity_change.bind(this));

    }

    disconnectedCallback(){
        console.log("disconnected")

        this.shadowRoot.querySelector("input").removeEventListener('change', this._quantity_change.bind(this));
    }

}

customElements.define('cac-quantity', InputQuantity);
