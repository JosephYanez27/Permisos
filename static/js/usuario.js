let pagina = 1;
let idUsuarioEdicion = null;

// 🔹 Cargar perfiles para filtro y modal
document.addEventListener("DOMContentLoaded", async () => {

    // 🔹 Buscar mientras se escribe
    document.getElementById("filtro-usuario")
    .addEventListener("keyup", () => {
        pagina = 1;
        buscarUsuarios();
    });

    // 🔹 Filtro por perfil
    document.getElementById("filtro-perfil")
    .addEventListener("change", () => {
        pagina = 1;
        buscarUsuarios();
    });

    // 🔹 Filtro por estado
    document.getElementById("filtro-estado")
    .addEventListener("change", () => {
        pagina = 1;
        buscarUsuarios();
    });
    await cargarPerfiles();
    // 🔹 Primera carga
    buscarUsuarios();
   

});

async function cargarUsuarios(page = 1){

 const usuario = document.getElementById("usuario").value;

 const res = await fetchAuth(
   `/usuario?page=${page}&usuario=${usuario}`
 );

 const data = await res.json();

 pintarTabla(data);
}


let totalPaginas = 1;

async function buscarUsuarios(){

 const usuario = document.getElementById("filtro-usuario").value.trim();
 const perfil = document.getElementById("filtro-perfil").value;
 const estado = document.getElementById("filtro-estado").value;

 let query = `/usuario?page=${pagina}`;

 if(usuario) query += `&usuario=${encodeURIComponent(usuario)}`;
 if(perfil) query += `&perfil=${perfil}`;
 if(estado) query += `&estado=${estado}`;

 try{

  const res = await fetchAuth(query);

  if(res.status === 401){
   alert("Sesión expirada");
   window.location.href = "/login.html";
   return;
  }

  const result = await res.json();
  

  console.log("Respuesta API:", result);

  if(!result || !result.data){
   console.error("Respuesta inválida:", result);
   renderTabla([]);
   return;
  }

  renderTabla(result.data);

  if(result.total !== undefined){
   calcularPaginas(result.total);
  }

 }catch(error){
  console.error("Error cargando usuarios:", error);
 }

}

function renderTabla(data){

 const tabla = document.getElementById("tabla-usuarios");
 tabla.innerHTML = "";

 if(data.length === 0){
  tabla.innerHTML = `<tr><td colspan="7">No hay datos</td></tr>`;
  return;
 }

 data.forEach(u => {

  tabla.innerHTML += `
  <tr>
   <td>${u.strnombreusuario ?? ""}</td>
   <td>${u.perfil ?? ""}</td>
   <td>${u.estado ?? ""}</td>
   <td>${u.strcorreo ?? ""}</td>
   <td>${u.strnumerocelular ?? ""}</td>

   <td>
     <button data-permiso="editar" onclick="editar(${u.id})">
       Editar
     </button>
   </td>

   <td>
     <button data-permiso="eliminar" onclick="eliminar(${u.id})">
       Eliminar
     </button>
   </td>

  </tr>
  `;
 });

 // 🔥 AQUI
 aplicarPermisosEnVista();
}
function calcularPaginas(total){

 const limit = 10;

 totalPaginas = Math.ceil(total / limit);

 let html = "";

 for(let i=1;i<=totalPaginas;i++){

  html += `<button onclick="irPagina(${i})">${i}</button>`;

 }

 document.getElementById("paginacion").innerHTML = html;
}

function irPagina(p){
 pagina = p;
 buscarUsuarios();
}



// 🔹 Eliminar
async function eliminar(id) {

    if (!confirm("¿Eliminar usuario?")) return;

    const response = await fetchAuth(`/usuario/${id}`, {
        method: "DELETE"
    });

    if (!response) return;

    alert("Usuario eliminado");
    buscarUsuarios();
}

// 🔹 Limpiar filtros
function limpiarFiltros() {

    document.getElementById("filtro-usuario").value = "";
    document.getElementById("filtro-perfil").value = "";
    document.getElementById("filtro-estado").value = "";

    pagina = 1;
    buscarUsuarios();
}

// 🔹 Abrir modal
function abrirModal() {

    idUsuarioEdicion = null;

    document.getElementById("usuario").value = "";
    document.getElementById("correo").value = "";
    document.getElementById("celular").value = "";

    document.getElementById("estado").value = "1"; // activo por defecto
    document.getElementById("password").disabled = false;
    document.getElementById("modalUsuario").style.display = "block";
}

// 🔹 Cerrar modal
function cerrarModal() {
    document.getElementById("modalUsuario").style.display = "none";
}

// 🔹 Guardar usuario
async function guardarUsuario() {

    const btn = document.querySelector(".btn-guardar");
    btn.disabled = true;

    const password = document.getElementById("password").value;

    const usuarioData = {

        strnombreusuario: document.getElementById("usuario").value.trim(),

        idperfil: parseInt(document.getElementById("perfil").value),

        strcorreo: document.getElementById("correo").value.trim(),

        strnumerocelular: document.getElementById("celular").value.trim(),

        idestadousuario: parseInt(document.getElementById("estado").value)
    };

    const esEdicion = !!idUsuarioEdicion;

    if (!esEdicion) {
        usuarioData.strpwd = password;
    }

    if (!validarUsuario(usuarioData, esEdicion)) {
        btn.disabled = false;
        return;
    }

    const metodo = esEdicion ? "PUT" : "POST";
    const endpoint = esEdicion
        ? `/usuario/${idUsuarioEdicion}`
        : "/usuario";

    try {

        const response = await fetchAuth(endpoint, {
            method: metodo,
            body: JSON.stringify(usuarioData)
        });

        if (response && response.ok) {

            mostrarMensaje(
                esEdicion
                    ? "✅ Usuario actualizado correctamente"
                    : "✅ Usuario creado correctamente",
                "success"
            );

            cerrarModal();

            // 🔥 RECARGA CON RETARDO
            setTimeout(() => {
                location.reload();
            }, 1200);

        } else {

            const error = await response.text();
            mostrarMensaje("❌ Error: " + error, "error");

        }

    } catch (e) {
        mostrarMensaje("❌ Error de conexión", "error");
    }

    btn.disabled = false;
}

// 🔹 Editar
async function guardarUsuario() {

    const btn = document.querySelector(".btn-guardar");
    btn.disabled = true;

    const password = document.getElementById("password").value;

    const usuarioData = {

        strnombreusuario: document.getElementById("usuario").value.trim(),

        idperfil: parseInt(document.getElementById("perfil").value),

        strcorreo: document.getElementById("correo").value.trim(),

        strnumerocelular: document.getElementById("celular").value.trim(),

        idestadousuario: parseInt(document.getElementById("estado").value)
    };

    const esEdicion = !!idUsuarioEdicion;

    if (!esEdicion) {
        usuarioData.strpwd = password;
    }

    if (!validarUsuario(usuarioData, esEdicion)) {
        btn.disabled = false;
        return;
    }

    const metodo = esEdicion ? "PUT" : "POST";
    const endpoint = esEdicion
        ? `/usuario/${idUsuarioEdicion}`
        : "/usuario";

    try {

        const response = await fetchAuth(endpoint, {
            method: metodo,
            body: JSON.stringify(usuarioData)
        });

        if (response && response.ok) {

            mostrarMensaje(
                esEdicion
                    ? "✅ Usuario actualizado correctamente"
                    : "✅ Usuario creado correctamente",
                "success"
            );

            cerrarModal();

            // 🔥 RECARGA CON RETARDO
            setTimeout(() => {
                location.reload();
            }, 1200);

        } else {

            const error = await response.text();
            mostrarMensaje("❌ Error: " + error, "error");

        }

    } catch (e) {
        mostrarMensaje("❌ Error de conexión", "error");
    }

    btn.disabled = false;
}
fetch("/menu.html")
.then(res => res.text())
.then(html => {
    document.getElementById("menu").innerHTML = html;
});
function validarUsuario(data, esEdicion = false) {

    // 🔹 Nombre
    if (!data.strnombreusuario || data.strnombreusuario.length < 3 || data.strnombreusuario.length > 30) {
        alert("El usuario debe tener entre 3 y 30 caracteres");
        return false;
    }

    const nombreRegex = /^[a-zA-Z0-9áéíóúÁÉÍÓÚñÑ\s]+$/;
    if (!nombreRegex.test(data.strnombreusuario)) {
        alert("El nombre contiene caracteres inválidos");
        return false;
    }

    // 🔹 Correo
    const correoRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    if (!correoRegex.test(data.strcorreo)) {
        alert("Correo inválido");
        return false;
    }

    // 🔹 Celular (opcional pero validado)
    if (data.strnumerocelular) {
        const celularRegex = /^[0-9]{10,15}$/;
        if (!celularRegex.test(data.strnumerocelular)) {
            alert("Celular inválido (solo números 10-15 dígitos)");
            return false;
        }
    }

    // 🔹 Password solo en creación
    if (!esEdicion) {
        if (!data.strpwd || data.strpwd.length < 6) {
            alert("La contraseña debe tener mínimo 6 caracteres");
            return false;
        }
    }

    return true;
}
async function cargarPerfiles() {

    try {

        const res = await fetchAuth("/perfil");

        if (!res.ok) {
            console.error("Error cargando perfiles");
            return;
        }

        const perfiles = await res.json();

        const selectPerfil = document.getElementById("perfil");
        const filtroPerfil = document.getElementById("filtro-perfil");

        selectPerfil.innerHTML = "";
        filtroPerfil.innerHTML = '<option value="">Todos</option>';

        perfiles.forEach(p => {

            selectPerfil.innerHTML += `
                <option value="${p.id}">
                    ${p.strnombreperfil}
                </option>
            `;

            filtroPerfil.innerHTML += `
                <option value="${p.id}">
                    ${p.strnombreperfil}
                </option>
            `;
        });

    } catch (error) {

        console.error("Error perfiles:", error);

    }
}
function calcularPaginas(total){

 const limit = 10;
 totalPaginas = Math.ceil(total / limit);

 let html = "";

 // 🔹 INICIO
 html += `
   <button onclick="irInicio()">⏮</button>
 `;

 // 🔹 ANTERIOR
 html += `
   <button onclick="irAnterior()">◀</button>
 `;

 // 🔹 PÁGINA ACTUAL
 html += `
   <span style="margin: 0 10px;">
     Página ${pagina} de ${totalPaginas}
   </span>
 `;

 // 🔹 SIGUIENTE
 html += `
   <button onclick="irSiguiente()">▶</button>
 `;

 // 🔹 FINAL
 html += `
   <button onclick="irFinal()">⏭</button>
 `;

 document.getElementById("paginacion").innerHTML = html;
}
function calcularPaginas(total){

 const limit = 10;
 totalPaginas = Math.ceil(total / limit);

 let html = "";

 // 🔹 INICIO
 html += `
   <button onclick="irInicio()">⏮</button>
 `;

 // 🔹 ANTERIOR
 html += `
   <button onclick="irAnterior()">◀</button>
 `;

 // 🔹 PÁGINA ACTUAL
 html += `
   <span style="margin: 0 10px;">
     Página ${pagina} de ${totalPaginas}
   </span>
 `;

 // 🔹 SIGUIENTE
 html += `
   <button onclick="irSiguiente()">▶</button>
 `;

 // 🔹 FINAL
 html += `
   <button onclick="irFinal()">⏭</button>
 `;

 document.getElementById("paginacion").innerHTML = html;
}
function irInicio(){
 pagina = 1;
 buscarUsuarios();
}

function irAnterior(){
 if(pagina > 1){
  pagina--;
  buscarUsuarios();
 }
}

function irSiguiente(){
 if(pagina < totalPaginas){
  pagina++;
  buscarUsuarios();
 }
}

function irFinal(){
 pagina = totalPaginas;
 buscarUsuarios();
}

async function subirFoto() {

    const fileInput = document.getElementById("foto");
    const file = fileInput.files[0];

    if (!file) {
        alert("Selecciona una imagen");
        return;
    }

    // 🔥 VALIDAR TIPO MIME
    if (!file.type.startsWith("image/")) {
        alert("Solo se permiten imágenes");
        fileInput.value = "";
        return;
    }

    // 🔥 VALIDAR EXTENSIÓN (extra seguridad)
    const extensionesValidas = ["jpg", "jpeg", "png", "webp"];
    const extension = file.name.split(".").pop().toLowerCase();

    if (!extensionesValidas.includes(extension)) {
        alert("Formato no permitido. Usa JPG, PNG o WEBP");
        fileInput.value = "";
        return;
    }

    // 🔥 VALIDAR TAMAÑO (ejemplo: 2MB)
    const maxSize = 2 * 1024 * 1024;
    if (file.size > maxSize) {
        alert("La imagen es demasiado grande (máx 2MB)");
        fileInput.value = "";
        return;
    }

    const formData = new FormData();
    formData.append("file", file);

    const usuario = JSON.parse(localStorage.getItem("usuario"));
const idUsuario = usuario?.id; // 🔥 dinámico

    const res = await fetchAuth(`/usuario/upload-foto/${idUsuario}`, {
        method: "POST",
        body: formData
    });

    alert(await res.text());
}
async function cargarFoto() {

    const usuario = JSON.parse(localStorage.getItem("usuario"));
const id = usuario?.id; // 🔥 luego lo haces dinámico

    const res = await fetchAuth(`/usuario/foto/${id}`);

    if (!res || !res.ok) {
        console.error("Error cargando foto");
        return;
    }

    const data = await res.json();

    document.getElementById("nombre").innerText = data.strnombreusuario;

    if (data.strfoto) {
        document.getElementById("img").src = API_URL.replace("/api", "") + data.strfoto;
    } else {
        document.getElementById("img").src = "https://via.placeholder.com/120";
    }
}


// 🔹 Exponer funciones
window.buscarUsuarios = buscarUsuarios;
window.editar = editar;
window.eliminar = eliminar;
window.limpiarFiltros = limpiarFiltros;
window.guardarUsuario = guardarUsuario;
window.abrirModal = abrirModal;
window.cerrarModal = cerrarModal;
