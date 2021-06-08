function shortUrl(url) {
    return fetch('/', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({ url })
    })
    .then(response => response.json())
}

function showResult(long, shorten) {
    const original = document.createElement('span')
    original.innerText = long + ' - '
    
    const link = document.createElement('a')
    link.innerHTML = shorten
    link.href = shorten
    link.target = "_blank"
    
    const container = document.createElement('div')
    container.append(original)
    container.appendChild(link)
    
    const results = document.getElementById('results')
    results.appendChild(container)
}

function handleSubmit(e) {
    e.preventDefault()

    const input = document.getElementById('url-input')
    const url = input.value
    
    if (!isValidURL(url)) {
        alert('Invalid url!')
        return
    }

    shortUrl(url)
        .then(data => showResult(url, data.url))
        .then(() => input.value = '')
        .catch(alert)
}

function isValidURL(url) {
    const pattern = new RegExp('^(https?:\\/\\/)?'+ // protocol
        '((([a-z\\d]([a-z\\d-]*[a-z\\d])*)\\.)+[a-z]{2,}|'+ // domain name
        '((\\d{1,3}\\.){3}\\d{1,3}))'+ // OR ip (v4) address
        '(\\:\\d+)?(\\/[-a-z\\d%_.~+]*)*'+ // port and path
        '(\\?[;&a-z\\d%_.~+=-]*)?'+ // query string
        '(\\#[-a-z\\d_]*)?$','i') // fragment locator
    
    return !!pattern.test(url)
}