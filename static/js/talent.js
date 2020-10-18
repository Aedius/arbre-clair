class TalentContainer extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: 'open' });

        this._point=13;
        this._talent=["a1", "b1", "d1"];

        this.shadowRoot.innerHTML = `
            <style>
                #parent{
                    position:relative
                }
                img{
                    width:100%
                }
                .top {
                    position: absolute;
                    left: 0px;
                    top: 0px;
                    z-index: 1;
                }
                .button{
                    z-index: 100;
                    cursor: pointer;
                }
                #a1 {
                    position: absolute;
                    top: 25%;
                    left: 9%;
                    width: 6%;
                    height: 15%;
                }
                #a2 {
                    position: absolute;
                    top: 58%;
                    left: 9%;
                    width: 6%;
                    height: 15%;
                }
                #b1 {
                    position: absolute;
                    top: 8%;
                    left: 17%;
                    width: 6%;
                    height: 15%;
                }
                #b2 {
                    position: absolute;
                    top: 41%;
                    left: 17%;
                    width: 6%;
                    height: 15%;
                }
                #b3 {
                    position: absolute;
                    top: 75%;
                    left: 17%;
                    width: 6%;
                    height: 15%;
                }
                #c1 {
                    position: absolute;
                    top: 22%;
                    left: 26%;
                    width: 6%;
                    height: 15%;
                }
                #c2 {
                    position: absolute;
                    top: 42%;
                    left: 26%;
                    width: 6%;
                    height: 15%;
                }
                #c3 {
                    position: absolute;
                    top: 60%;
                    left: 26%;
                    width: 6%;
                    height: 15%;
                }
                #d1 {
                    position: absolute;
                    top: 8%;
                    left: 35%;
                    width: 6%;
                    height: 15%;
                }
                #d2 {
                    position: absolute;
                    top: 42%;
                    left: 35%;
                    width: 6%;
                    height: 15%;
                }
                #d3 {
                    position: absolute;
                    top: 75%;
                    left: 35%;
                    width: 6%;
                    height: 15%;
                }
                #e1 {
                    position: absolute;
                    top: 25%;
                    left: 43%;
                    width: 6%;
                    height: 15%;
                }
                #e2 {
                    position: absolute;
                    top: 42%;
                    left: 43%;
                    width: 6%;
                    height: 15%;
                }
                #e3 {
                    position: absolute;
                    top: 60%;
                    left: 43%;
                    width: 6%;
                    height: 15%;
                }
                #f1 {
                    position: absolute;
                    top: 9%;
                    left: 51%;
                    width: 6%;
                    height: 15%;
                }
                #f2 {
                    position: absolute;
                    top: 75%;
                    left: 51%;
                    width: 6%;
                    height: 15%;
                }
                #g1 {
                    position: absolute;
                    top: 9%;
                    left: 69%;
                    width: 6%;
                    height: 15%;
                }
                #g2 {
                    position: absolute;
                    top: 42%;
                    left: 69%;
                    width: 6%;
                    height: 15%;
                }
                #g3 {
                    position: absolute;
                    top: 76%;
                    left: 69%;
                    width: 6%;
                    height: 15%;
                }
                #h1 {
                    position: absolute;
                    top: 0%;
                    left: 77%;
                    width: 6%;
                    height: 15%;
                }
                #h2 {
                    position: absolute;
                    top: 17%;
                    left: 77%;
                    width: 6%;
                    height: 15%;
                }
                #h3 {
                    position: absolute;
                    top: 33%;
                    left: 77%;
                    width: 6%;
                    height: 15%;
                }
                #h4 {
                    position: absolute;
                    top: 51%;
                    left: 77%;
                    width: 6%;
                    height: 15%;
                }
                #h5 {
                    position: absolute;
                    top: 67%;
                    left: 77%;
                    width: 6%;
                    height: 15%;
                }
                #h6 {
                    position: absolute;
                    top: 84%;
                    left: 77%;
                    width: 6%;
                    height: 15%;
                }
                #i1 {
                    position: absolute;
                    top: 10%;
                    left: 85%;
                    width: 6%;
                    height: 11%;
                }
                #i2 {
                    position: absolute;
                    top: 44%;
                    left: 85%;
                    width: 6%;
                    height: 11%;
                }
                #i3 {
                    position: absolute;
                    top: 78%;
                    left: 85%;
                    width: 6%;
                    height: 11%;
                }
                #j1 {
                    position: absolute;
                    top: 0%;
                    left: 89%;
                    width: 6%;
                    height: 10%;
                }
                #j2 {
                    position: absolute;
                    top: 10%;
                    left: 93%;
                    width: 6%;
                    height: 11%;
                }
                #j3 {
                    position: absolute;
                    top: 21%;
                    left: 89%;
                    width: 6%;
                    height: 10%;
                }
                #j4 {
                    position: absolute;
                    top: 34%;
                    left: 89%;
                    width: 6%;
                    height: 10%;
                }
                #j5 {
                    position: absolute;
                    top: 44%;
                    left: 93%;
                    width: 6%;
                    height: 11%;
                }
                #j6 {
                    position: absolute;
                    top: 55%;
                    left: 89%;
                    width: 6%;
                    height: 10%;
                }
                #j7 {
                    position: absolute;
                    top: 68%;
                    left: 89%;
                    width: 6%;
                    height: 10%;
                }
                #j8 {
                    position: absolute;
                    top: 78%;
                    left: 93%;
                    width: 6%;
                    height: 11%;
                }
                #j9 {
                    position: absolute;
                    top: 89%;
                    left: 89%;
                    width: 6%;
                    height: 10%;
                }
            </style
            <div>
                <div id="parent">
                    <img src="/img/talent/base.png" />
                    <div class="button" id="a1"></div>
                    <div class="button" id="a2"></div>
                    <div class="button" id="b1"></div>
                    <div class="button" id="b2"></div>
                    <div class="button" id="b3"></div>
                    <div class="button" id="c1"></div>
                    <div class="button" id="c2"></div>
                    <div class="button" id="c3"></div>
                    <div class="button" id="d1"></div>
                    <div class="button" id="d2"></div>
                    <div class="button" id="d3"></div>
                    <div class="button" id="e1"></div>
                    <div class="button" id="e2"></div>
                    <div class="button" id="e3"></div>
                    <div class="button" id="f1"></div>
                    <div class="button" id="f2"></div>
                    <div class="button" id="g1"></div>
                    <div class="button" id="g2"></div>
                    <div class="button" id="g3"></div>
                    <div class="button" id="h1"></div>
                    <div class="button" id="h2"></div>
                    <div class="button" id="h3"></div>
                    <div class="button" id="h4"></div>
                    <div class="button" id="h5"></div>
                    <div class="button" id="h6"></div>
                    <div class="button" id="i1"></div>
                    <div class="button" id="i2"></div>
                    <div class="button" id="i3"></div>
                    <div class="button" id="j1"></div>
                    <div class="button" id="j2"></div>
                    <div class="button" id="j3"></div>
                    <div class="button" id="j4"></div>
                    <div class="button" id="j5"></div>
                    <div class="button" id="j6"></div>
                    <div class="button" id="j7"></div>
                    <div class="button" id="j8"></div>
                    <div class="button" id="j9"></div>

                    <div id="other">
                    </div>
                </div>
            <div>
        `

        this._other = this.shadowRoot.querySelector("#other");

        var buttonList = this.shadowRoot.querySelectorAll(".button")
        for (var i = 0; i < buttonList.length; i++) {
            buttonList[i].addEventListener('click', this._click.bind(this));
        }

    }

    connectedCallback() {
        this._display();
    }

    _click(event){
        const id = event.target.id

        console.log(id)

        this._talent.push(id)
        this._display()
    }

    _display(){

        console.log(this._talent)
        const imgList = this._talent.map( code => {
            return `<img class="top" src="/img/talent/${code}.png" />`
        })

        this._other.innerHTML = imgList.join('');

    }

}
customElements.define('cac-talent-container', TalentContainer);