const { invoke } = window.__TAURI__.tauri;

const form = document.getElementById('schoolForm')
const schoolsElem = document.querySelector('.schools')

form.addEventListener('submit', async(e) => {
  e.preventDefault()
  form.hidden = true
  invoke("search_school", { q: document.getElementById('schoolName').value }).then((s) => {
    let schools = JSON.parse(s).data;

    schoolsElem.innerHTML = ''
    schools.forEach(school => {
      school = school.attributes;
      const schooHTML = `<b class="school-name" title="${school.id}">${school.name}</b>
      <span class="school-ems">(${school.emsCode})</span> -
      <i class="school-city">${school.city.replace('CEDEX', '').trim()} ${school.zipCode}</i>
      <button class="school-select">Choisir</button>`

      const li = document.createElement('li')
      li.innerHTML = schooHTML
      li.querySelector('button').addEventListener('click', () => invoke("auth_school", { school }).then((url) => { window.location.href = url; }))
      schoolsElem.appendChild(li)
    });
  }).catch((err) => {
    console.error(err);
  });
})