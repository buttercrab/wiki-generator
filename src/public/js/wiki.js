let search_data = {};
let selected = null;
let input = document.getElementById('t-search-input');
let button = document.getElementById('t-search-button');
let result = document.getElementById('t-search-result');
let list = [];

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

(function fetch_search_data() {
    let r = new XMLHttpRequest();
    r.open('GET', '/r/search.json');
    r.send();
    r.onload = function () {
        search_data = JSON.parse(r.response);
    };

    input.addEventListener('keydown', function (event) {
        if (event.which === 13 || event.keyCode === 13 || event.key === "Enter") {
            goto(list[selected || 0]);
        } else if (event.which === 40 || event.keyCode === 40 || event.key === "ArrowDown") {
            event.preventDefault();
            if (selected === null) {
                selected = 0;
            } else {
                selected++;
            }
            if (selected >= list.length) selected = null;
            updateList();
        } else if (event.which === 38 || event.keyCode === 38 || event.key === "ArrowUp") {
            event.preventDefault();
            if (selected === null) {
                selected = list.length - 1;
            } else {
                selected--;
            }
            if (selected < 0) selected = null;
            updateList();
        }
    });

    input.addEventListener('input', function () {
        updateList();
    });

    button.addEventListener('click', function () {
        goto(list[selected || 0]);
    });
})();