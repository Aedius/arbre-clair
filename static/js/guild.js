class Guild extends HTMLElement {

    constructor() {
        super();
        this.attachShadow({ mode: 'open' });
    }

  connectedCallback() {

    var _tag = this.getAttribute('tag');
    var _name = this.getAttribute('name');
    var _title = this.getAttribute('name');

    this.shadowRoot.innerHTML = `
    <style>
        img {
            float: left;
            width:10%
        }
        div {
            clear: left;
            padding-top: 20px;
        }
        span {
            font-weight: bold;
        }
        p {
            margin-left : 15%;
        }
    </style>
    <div>
        <img src="/img/guild/${_tag}.png" alt="${_name}">
        <p>
        <span>${_name}</span>
        <br/>
        // ${_tag}
        </p>
        <p>
        Relation : <slot></slot>
        </p>
    </div>
    `;
  }

}
customElements.define('cac-guild', Guild);