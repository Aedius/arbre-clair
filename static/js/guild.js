class Guild extends HTMLElement {

    constructor() {
        super();
        this.attachShadow({ mode: 'open' });

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
                .name {
                    font-size: 1.8em;
                    color: ${StrongColor}
                }
                p {
                    color: black;
                    margin-left : 15%;
                }
            </style>
            <div>
                <img src="/img/logo.png" alt="name" style="float:left">
                <img src="/img/logo.png" alt="name" style="float:right">
                <p>
                <span class="name">name</span> <span class="tag">tag</span>
                </p>
                <p>
                Relation : <slot></slot>
                </p>
            </div>
            `;
    }

  connectedCallback() {

// <img src="/img/guild/${_tag}.png" alt="${_name}" style="float:right">

    var _tag = this.getAttribute('tag');
    var _name = this.getAttribute('name');
    var _title = this.getAttribute('name');

    var nodeName = this.shadowRoot.querySelector(".name");
    nodeName.innerHTML = _name;

    var nodeTag = this.shadowRoot.querySelector(".tag");
    nodeTag.innerHTML = "// "+_tag;

    var nodeImg = this.shadowRoot.querySelectorAll("img");
    for (var i = 0; i < nodeImg.length; i++) {
      nodeImg[i].src = `/img/guild/${_tag}.png`
      nodeImg[i].alt = _name
    }
    console.log(nodeImg)

  }

}
customElements.define('cac-guild', Guild);