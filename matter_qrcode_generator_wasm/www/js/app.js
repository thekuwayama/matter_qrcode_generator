// import
import init, {do_print_qr} from '../../pkg/matter_qrcode_generator_wasm.js'
import '../css/style.css'

// function
export function printQR() {
    init().then(() => {
        const vid = parseInt(document.getElementById('vid').value)
        const pid = parseInt(document.getElementById('pid').value)
        const passcode = parseInt(document.getElementById('passcode').value)
        const discriminator = parseInt(document.getElementById('discriminator').value)
        const softAP = document.getElementById('softAP').checked
        const ble = document.getElementById('ble').checked
        const onIPNW = document.getElementById('onIPNW').checked
        const w = window.outerWidth / 5

        var qr
        try {
            qr = do_print_qr(vid, pid, passcode, discriminator, softAP, ble, onIPNW, w)
        } catch(e) {
            alert(e)
            return
        }

        if (qr == null) {
            return
        }

        document.getElementById('qr').innerHTML = qr
        document.getElementById('download').style.display = ''
    })
}

export function downloadSVG() {
    const svg = document.getElementById('qr').innerHTML
    const blob = new Blob([svg.toString()])
    const a = document.createElement('a')
    a.download = 'qr.svg'
    a.href = window.URL.createObjectURL(blob)

    document.body.appendChild(a)
    a.click()
    a.remove()
}

// init
document.getElementById('print').onclick = printQR
document.getElementById('download').onclick = downloadSVG
