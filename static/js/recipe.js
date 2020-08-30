class RecipeList extends HTMLElement {

    constructor() {
        super();

        this._data=null

        this.attachShadow({ mode: 'open' });
    }

    connectedCallback() {
        this._kind = this.getAttribute('kind');
        this._name = this.getAttribute('name');

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

          let li = "loading";
          if (this._data != null) {
           li = this._data.map( recipe => {
                console.log(recipe)
               return  `<li><cac-link href="pages/recipe.html#${this._kind}/${recipe.key}" >${recipe.name} :</cac-link>
                    ${recipe.stat}</li>`
            })
          }
          console.log(li)

          this.shadowRoot.innerHTML = `
            <style>
                ul{
                    list-style-type: none;
                }
            </style>
            <div>
                <h2>${this._name}</h2>
                <ul>
                    ${li.join('')}
                </ul>
            </div>
          `

      }

}
customElements.define('cac-recipe-list', RecipeList);