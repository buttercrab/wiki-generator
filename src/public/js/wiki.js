let search_data = {};

let selected = null;
let input = document.getElementById('t-search-input');
let button = document.getElementById('t-search-button');
let result = document.getElementById('t-search-result');
let list = [];

let m_selected = null;
let m_toggle = document.getElementById('t-m-search-toggle-button');
let m_input = document.getElementById('t-m-search-input');
let m_button = document.getElementById('t-m-search-button');
let m_result = document.getElementById('t-m-search-result');
let search_toggle = false;
let m_list = [];

// code from https://github.com/hakimel/css/tree/master/progress-nav
// thanks for great code!
//
// table of contents
(function toc() {
    let toc = document.querySelector('.toc');
    let tocPath = document.querySelector('.toc-marker path');
    let tocItems;

    // Factor of screen size that the element must cross
    // before it's considered visible
    let TOP_MARGIN = 0.05, BOTTOM_MARGIN = 0;
    let pathLength;
    let lastPathStart, lastPathEnd;

    window.addEventListener('resize', drawPath, false);
    window.addEventListener('scroll', sync, false);
    drawPath();

    function drawPath() {
        tocItems = [].slice.call(toc.querySelectorAll('li'));

        // Cache element references and measurements
        tocItems = tocItems.map(function (item) {
            let anchor = item.querySelector('a');
            let target = document.getElementById(anchor.getAttribute('href').slice(1));

            return {
                listItem: item,
                anchor: anchor,
                target: target
            };
        });

        // Remove missing targets
        tocItems = tocItems.filter(function (item) {
            return !!item.target;
        });

        let path = [];
        let pathIndent;

        tocItems.forEach(function (item, i) {
            let x = item.anchor.offsetLeft - 5,
                y = item.anchor.offsetTop,
                height = item.anchor.offsetHeight;

            if (i === 0) {
                path.push('M', x, y, 'L', x, y + height);
                item.pathStart = 0;
            } else {
                // Draw an additional line when there's a change in
                // indent levels
                if (pathIndent !== x) path.push('L', pathIndent, y);

                path.push('L', x, y);

                // Set the current path so that we can measure it
                tocPath.setAttribute('d', path.join(' '));
                item.pathStart = tocPath.getTotalLength() || 0;

                path.push('L', x, y + height);
            }

            pathIndent = x;

            tocPath.setAttribute('d', path.join(' '));
            item.pathEnd = tocPath.getTotalLength();
        });

        pathLength = tocPath.getTotalLength();

        sync();
    }

    function sync() {
        let windowHeight = window.innerHeight;
        let pathStart = pathLength, pathEnd = 0;
        let visibleItems = 0;

        tocItems.forEach(function (item) {
            let targetBounds = item.target.getBoundingClientRect();

            if (targetBounds.bottom > windowHeight * TOP_MARGIN && targetBounds.top < windowHeight * (1 - BOTTOM_MARGIN)) {
                pathStart = Math.min(item.pathStart, pathStart);
                pathEnd = Math.max(item.pathEnd, pathEnd);

                visibleItems += 1;

                item.listItem.classList.add('visible');
            } else {
                item.listItem.classList.remove('visible');
            }
        });

        // Specify the visible path or hide the path altogether
        // if there are no visible items
        if (visibleItems > 0 && pathStart < pathEnd) {
            if (pathStart !== lastPathStart || pathEnd !== lastPathEnd) {
                tocPath.setAttribute('stroke-dashoffset', '1');
                tocPath.setAttribute('stroke-dasharray', '1, ' + pathStart + ', ' + (pathEnd - pathStart) + ', ' + pathLength);
                tocPath.setAttribute('opacity', 1);
            }
        } else {
            tocPath.setAttribute('opacity', 0);
        }

        lastPathStart = pathStart;
        lastPathEnd = pathEnd;
    }
})();

function goto(title) {
    window.location = "/w/" + title;
}

function updateList() {
    let text = input.value.toLowerCase();
    let res = search_data[text];
    if (res) {
        res.slice(0, 10);
        if (list !== res) selected = null;
        list = res;

        let html = `<div class="search-result">`;
        for (let i = 0; i < list.length; i++) {
            if (selected === i) {
                html += `<div class="selected" onmouseout="selected = null;updateList();" onclick="goto('` + list[i] + `')">` + list[i] + `</div>`;
            } else {
                html += `<div onmouseover="selected = ` + i + `;updateList();" onclick="goto('` + list[i] + `')">` + list[i] + `</div>`;
            }
        }
        html += `</div>`;
        result.innerHTML = html;
    } else {
        list = [];
        result.innerHTML = '';
    }
}

function updateMobileList() {
    let text = m_input.value.toLowerCase();
    let res = search_data[text];
    if (res) {
        res.slice(0, 10);
        if (list !== res) m_selected = null;
        m_list = res;

        let html = `<div class="search-result-mobile">`;
        for (let i = 0; i < m_list.length; i++) {
            if (m_selected === i) {
                html += `<div class="selected" onmouseout="m_selected = null;updateMobileList();" onclick="goto('` + m_list[i] + `')">` + m_list[i] + `</div>`;
            } else {
                html += `<div onmouseover="m_selected = ` + i + `;updateMobileList();" onclick="goto('` + m_list[i] + `')">` + m_list[i] + `</div>`;
            }
        }
        html += `</div>`;
        m_result.innerHTML = html;
    } else {
        list = [];
        m_result.innerHTML = '';
    }
}

function onKey(event, id, key, callback) {
    if (event.which === id || event.keyCode === id || event.key === key) {
        event.preventDefault();
        callback();
    }
}

(function fetch_search_data() {
    let r = new XMLHttpRequest();
    r.open('GET', '/r/search.json');
    r.send();
    r.onload = function () {
        search_data = JSON.parse(r.response);
    };

    input.addEventListener('keydown', function (event) {
        onKey(event, 13, "Enter", function () {
            if (input.value.length !== 0) {
                goto(list[selected || 0] || input.value);
            }
        });

        onKey(event, 40, "ArrowDown", function () {
            if (selected === null) {
                selected = 0;
            } else {
                selected++;
            }
            if (selected >= list.length) selected = null;
            updateList();
        });

        onKey(event, 38, "ArrowUp", function () {
            if (selected === null) {
                selected = list.length - 1;
            } else {
                selected--;
            }
            if (selected < 0) selected = null;
            updateList();
        })

        onKey(event, 27, "Escape", function () {
            input.blur();
        });
    });

    input.addEventListener('input', function () {
        updateList();
    });

    button.addEventListener('click', function () {
        if (input.value.length !== 0) {
            goto(list[selected || 0] || input.value);
        }
    });

    m_input.addEventListener('keydown', function (event) {
        onKey(event, 13, "Enter", function () {
            if (m_input.value.length !== 0) {
                goto(m_list[m_selected || 0] || m_input.value);
            }
        });

        onKey(event, 40, "ArrowDown", function () {
            if (m_selected === null) {
                m_selected = 0;
            } else {
                m_selected++;
            }
            if (m_selected >= m_list.length) m_selected = null;
            updateMobileList();
        });

        onKey(event, 38, "ArrowUp", function () {
            if (m_selected === null) {
                m_selected = m_list.length - 1;
            } else {
                m_selected--;
            }
            if (m_selected < 0) m_selected = null;
            updateMobileList();
        })

        onKey(event, 27, "Escape", function () {
            m_input.blur();
        });
    });

    m_input.addEventListener('input', function () {
        updateMobileList();
    });

    m_button.addEventListener('click', function () {
        if (m_input.value.length !== 0) {
            goto(m_list[m_selected || 0] || m_input.value);
        }
    });

    m_toggle.addEventListener('click', function () {
        if (search_toggle) {
            document.getElementsByClassName('search-mobile-wrap')[0].style.display = 'none';
            m_toggle.innerHTML = `<i class="fas fa-search"></i>`;
            search_toggle = false;
        } else {
            document.getElementsByClassName('search-mobile-wrap')[0].style.display = 'block';
            m_toggle.innerHTML = `<i class="fas fa-times"></i>`;
            search_toggle = true;
        }
    });
})();

document.addEventListener('keydown', function (event) {
    if (document.activeElement !== input) {
        onKey(event, 191, "/", function () {
            input.focus();
        });
    }
});
