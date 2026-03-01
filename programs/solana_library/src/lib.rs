use anchor_lang::prelude::*;
// ID del Solana Program, este espacio se llena automaticamente al hacer el "build"
declare_id!("H7UEbP7T4SMUvwgXnfvUNCUptsYh9P27GA58ens5L9hU");

#[program] // Macro que convierte codigo de Rust a Solana. Apartir de aqui empieza tu codigo!
pub mod biblioteca {
    use super::*; // Importa todas los structs y enums definidos fuera del modulo

    //////////////////////////// Instruccion: Crear Biblioteca /////////////////////////////////////
    /*
    Permite la creacion de una PDA (Program Derived Adress), un tipo especial de cuenta en solana que permite prescindir 
    del uso de llaves privadas para la firma de transacciones. 

    Esta cuenta contendra el objeto (struct) de tipo Biblioteca donde podremos almacenar los Libros. 
    La creacion de la PDA depende de 3 cosas:
        * Wallet address 
        * Program ID 
        * string representativo, regularmente relacionado con el nombre del proyecto
    
    La explicacion de esto continua en el struct NuevaBiblioteca

    Parametros de entrada:
        * nombre -> nombre de la biblioteca -> tipo string
     */
    pub fn crear_biblioteca(context: Context<NuevaBiblioteca>, nombre: String) -> Result<()> {
        // "Context" siempre suele ir como primer parametro, ya que permite acceder al objeto o cuenta con el que queremos interactuar
        // Dentro del context va al tipo de objeto o cuenta con el que deseamos interactuar. 
        let owner_id = context.accounts.owner.key(); // Accedemos al wallet address del caller 
        msg!("Owner id: {}", owner_id); // Print de verificacion

        let libros: Vec<Libro> = Vec::new(); // Crea un vector vacio 

        // Creamos un Struct de tipo biblioteca y lo guardamos directamente 
        context.accounts.biblioteca.set_inner(Biblioteca { 
            owner: owner_id,
            nombre,
            libros,
        });
        Ok(()) // Representa una transaccion exitosa 
    }

    //////////////////////////// Instruccion: Agregar Libro /////////////////////////////////////
    /*
    Agrega un libro al vector de libros ontenido en el struct Biblioteca. 
    En este caso el contexto empleado es el struct NuevoLibro. Mientras que NuevaBiblioteca permite crear 
    Instancias de una Biblioteca. NuevoLibro permite crear y modificar los valores relacionados a cualquier
    struct de tipo Libro.

    Parametros de entrada:
        * nombre -> nombre del libro -> string
        * paginas -> numero de paginas del libro -> u16
     */ 
    pub fn agregar_libro(context: Context<NuevoLibro>, nombre: String, paginas: u16) -> Result<()> {
        require!( // Medida de seguridad para identificar que SOLO el owner de la biblioteca sea el que hace cambios en ella
            context.accounts.biblioteca.owner == context.accounts.owner.key(), // Condicion, true -> continua, false -> error
            Errores::NoEresElOwner // Codigo de error, ver enum Errores
        ); 

        let libro = Libro { // Creacion de un struct tipo Libro
            nombre,
            paginas,
            disponible: true,
        };

        context.accounts.biblioteca.libros.push(libro); // Agrega el Libro al vector de libros de Biblioteca

        Ok(()) // Transaccion exitosa
    }

    //////////////////////////// Instruccion: Eliminar Libro /////////////////////////////////////
    /*
    Elimina un libro apartir de su nombre. Error si libro no existe, Error si vector vacio. 

    Parametros de entrada:
        * nombre -> Nombre del libro -> string
     */
    pub fn eliminar_libro(context: Context<NuevoLibro>, nombre: String) -> Result<()> {
        require!( // Medida de seguridad
            context.accounts.biblioteca.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let libros = &mut context.accounts.biblioteca.libros; // Referencia mutable al vector de libros

        for i in 0..libros.len() { // Se itera mediante el indice todo el contenido del vector en busca del libro a eliminar
            if libros[i].nombre == nombre { // Si lo encuentra prodece a borrarlo mediante el metodo remove
                libros.remove(i);
                msg!("Libro {} eliminado!", nombre); // Mensaje de borrado exitoso
                return Ok(()); // Transaccion exitosa
            }
        }
        Err(Errores::LibroNoExiste.into()) // Transaccion fallida, nunca encontro el libro
    }

    //////////////////////////// Instruccion: Ver Libros /////////////////////////////////////
    /*
    Muestra en el log de la transaccion el contenido completo del vector de libros de la Biblioteca

    Parametros de entrada:
        Ninguno
     */
    pub fn ver_libros(context: Context<NuevoLibro>) -> Result<()> {
        require!( // Medida de seguridad 
            context.accounts.biblioteca.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        // :#? requiere que NuevoLibro tenga atributo Debug. Permite la visualizacion completa del vector en el log
        msg!("La lista de libros actualmente es: {:#?}", context.accounts.biblioteca.libros); // Print en log
        Ok(()) // Transaccion exitosa 
    }

    
    //////////////////////////// Instruccion: Alternar Estado /////////////////////////////////////
    /* 
    Cambia el estado de disponible de false a true o de true a false.

    Parametros de entrada:
        * nombre -> Nombre del libro -> string
     */
    pub fn alternar_estado(context: Context<NuevoLibro>, nombre: String) -> Result<()> {
        require!( // Medida de seguridad
            context.accounts.biblioteca.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let libros = &mut context.accounts.biblioteca.libros; // Referencia mutable al vector de libros
        for i in 0..libros.len() { // Se itera mediante el indice el vector de libros
            let estado = libros[i].disponible;  // Se almacena el estado del vector actual

            if libros[i].nombre == nombre { // Si ecuentra el nombre del libro procede a cambiar el valor del estado 
                let nuevo_estado = !estado;
                libros[i].disponible = nuevo_estado;
                msg!("El libro: {} ahora tiene un valor de disponibilidad: {}", nombre, nuevo_estado); // log print de la nueva disponibilidad
                return Ok(()); // Transaccion exitosa
            }
        }

        Err(Errores::LibroNoExiste.into()) // Transaccion fallida, libro no existe
    }

}

/*
Codigos de error
Todos los codigos se almacenan en un enum con la siguiente estructura:
#[msg("MENSAJE DE ERROR")] (dentro de las comillas)
NombreDelError, (En camel case)
*/
#[error_code]
pub enum Errores {
    #[msg("Error, no eres el propietario de la biblioteca que deseas modificar")]
    NoEresElOwner,
    #[msg("Error, el libro con el que deseas interactuar no existe")]
    LibroNoExiste,
}

#[account] // Especifica que el strcut es una cuenta que se almacenara en la blockchain
#[derive(InitSpace)] // Genera la constante INIT_SPACE y determina el espacio de almacenamiento necesario 
pub struct Biblioteca { // Define la Biblioteca
    owner: Pubkey, // Pubkey es un formato de llave publica de 32 bytes 

    #[max_len(60)] // Cantidad maxima de caracteres del string: nombre
    nombre: String,

    #[max_len(10)] // Tamaño maximo del vector libros 
    libros: Vec<Libro>,
}

/*
Struct interno o secundario (No es una cuenta). Se define por derive y cuenta con los siguientes atributos:
    * AnchorSerialize -> Permite guardar el struct en la cuenta 
    * AnchorDeserialize -> Permite leer su contenido desde la cuenta 
    * Clone -> Para copiar su contenido o valores 
    * InitSpace -> Calcula el tamaño necesario para ser almacenado en la blockchain
    * PartialEq -> Para usar sus valores y compararlos con "=="
    * Debug -> Para mostrarlo en log con ":?" o ":#?"
*/
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Libro {
    #[max_len(60)]
    nombre: String,

    // Los siguientes datos no rquieren de max_len porque ya estan definidos (numero de 16 bits y false o true)
    paginas: u16, 

    disponible: bool,
}


// Creacion de los contextos para las instrucciones (funciones)
#[derive(Accounts)] // Especifica que este struct describe las cuentas que se requieren para determinada instruccion
pub struct NuevaBiblioteca<'info> { // contexto de la instruccion
    #[account(mut)] 
    pub owner: Signer<'info>, // Se define que el owner como el que pagara la transaccion, por eso es mut, para que cambie el balance de la cuenta

    #[account(
        init, // Inidica que al llamar la instruccuion se creara una cuenta
        // puede ser remplazado por "init_if_needed" para que solo se cree una vez por caller
        payer = owner, // Se especifica que quien paga el llamado a la instruccion, en este caso llama la instruccion 
        space = Biblioteca::INIT_SPACE + 8, // Se calcula el espacio requerido para almacenar el Solana Program On-Chain
        seeds = [b"biblioteca", owner.key().as_ref()], // Se especifica que la cuenta es una PDA que depende de un string y el id del owner
        bump // Metodo para determinar el el id de la biblioteca en base a lo anterior 
    )]
    pub biblioteca: Account<'info, Biblioteca>, // Se especifica que la cuenta creada (PDA) almacenara la biblioteca 

    pub system_program: Program<'info, System>, // Programa necesario para crear la cuenta 
}

// Contexto para la creacion y modificacion de libros 
#[derive(Accounts)] // Especifica que este struct se requiere para todas las instrucciones relacionadas con la creacion o modificacion de Libro
pub struct NuevoLibro<'info> {
    pub owner: Signer<'info>, // El owner de la cuenta es quien paga la transaccion

    #[account(mut)] 
    pub biblioteca: Account<'info, Biblioteca>, // Se marca biblioteca como mutable porque se modificara tanto el vector como los libros que contiene
}