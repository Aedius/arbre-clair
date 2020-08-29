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
            max-height:90%
        }
        div {
            width: 100%;
            display: block;
            height: 100px;
            padding: 10px;
            margin-bottom: 10px;
            border-top: 2mm ridge rgba(241,81,85,1);
            border-left: 2mm ridge rgba(241,81,85,1);
            border-right: 2mm ridge rgba(241,81,85,1);
            border-radius: 0% 0% 20% 20%;
            background-color: #fff;
        }
        span {
            font-size: 1.8em;
            color: ${StrongColor}
        }
        p {
            color: black;
            margin-left : 15%;
        }
    </style>
    <div>
        <img src="/img/guild/${_tag}.png" alt="${_name}" style="float:left">
        <img src="/img/guild/${_tag}.png" alt="${_name}" style="float:right">
        <p>
        <span>${_name}</span> // ${_tag}
        </p>
        <p>
        Relation : <slot></slot>
        </p>
    </div>
    `;
  }

}
customElements.define('cac-guild', Guild);