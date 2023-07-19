window.addEventListener("load", function(){
    //获取dom
    const p = document.querySelector("p")
    const button = document.querySelector("button")

    //为button添加点击事件
    button.addEventListener("click", detectWebGLContext, false)
    function detectWebGLContext(){
        const canvas = document.createElement("canvas")
        let gl = canvas.getContext("webgl")
        
    }
})