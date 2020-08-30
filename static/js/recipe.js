class RecipeContainer extends HTMLElement {

    constructor() {
        super();

        this._craft = [{
                code : "cooking",
                name : "Cuisine",
            }, {
                code : 'foo',
                name : "coming soon",
            }, {
                code : 'foo2',
                name : "cooming soon",
            }
        ]

        this._data=null

        this.attachShadow({ mode: 'open' });
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

          const li = this._craft.map( craft => {
             if ( craft.code == _selected){
                 return `<li><cac-link href="/pages/metiers.html#${craft.code}" current="1" data-hover="${craft.name}">${craft.name}</cac-link></li>`
             }else{
                return  `<li><cac-link href="/pages/metiers.html#${craft.code}" data-hover="${craft.name}">${craft.name}</cac-link></li>`
             }
         })

         if (_selected != ""){
            console.log(_recipe)

             if (_recipe == ""){
                _content = `
                    <cac-recipe-list kind="${_selected}"></cac-recipe-list>
                `
             }else{
                _content= `
                   <cac-recipe kind="${_recipe}"></cac-recipe>
                `
             }
         }

          this.shadowRoot.innerHTML = `
            <style>
                ul{
                    list-style-type: none;
                }
                li {
                     display: inline-block;
                     margin: 0 1.5em;
                }
            </style>
            <nav>
                <ul>
                    ${li.join('')}
                </ul>
            </nav>
            ${_content}
          `
      }

}
customElements.define('cac-recipe-container', RecipeContainer);

class RecipeList extends HTMLElement {

    constructor() {
        super();
        this._data=null

        this.attachShadow({ mode: 'open' });
    }

    connectedCallback() {
        this._kind = this.getAttribute('kind');

        var those = this;

        fetch('/api/recipe/' + this._kind).then(function (response) {
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

        this._display()
    }

      _display(){

          let li = ["loading"];
          if (this._data != null) {
           li = this._data.map( recipe => {
               return  `<li><cac-link href="/pages/metiers.html#${this._kind}/${recipe.key}" >${recipe.name} :</cac-link>
                    ${recipe.stat}</li>`
            })
          }

          this.shadowRoot.innerHTML = `
            <style>
                ul{
                    list-style-type: none;
                }
            </style>
            <div>
                <ul>
                    ${li.join('')}
                </ul>
            </div>
          `

      }

}
customElements.define('cac-recipe-list', RecipeList);


class Recipe extends HTMLElement {

    constructor() {
        super();

        this._data=null

        this.attachShadow({ mode: 'open' });
    }

    connectedCallback() {
        this._kind = this.getAttribute('kind');

        this.shadowRoot.innerHTML = `
        <div>
            ${this._kind}
        </div>
      `
    }
}
customElements.define('cac-recipe', Recipe);