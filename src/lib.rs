//libreria de ancor -> prelude
use anchor_lang::prelude::*;
//lo guardamos en una cuenta
declare_id!("");

#[program]

//modulo de biblioteca, lo definimos con mod de manera publica
pub mod biblioteca {
    //importamos la libreria super
    use super::*;

    //creamos una funcion "crear"
    pub fn crear_biblioteca() -> Result<()>{
        //codigo
    }
}

//Le indicamos a solana que va a ser utilizado por una cuenta
#[account]
//definicion de espacios
#[derive(InitSpace)] //libreria
//creamos una estructura
pub struct Biblioteca{
    dueno: Pubkey, //Pubkey significa llave publica
    #[max_len(60)] //espacio de 60 para el nombre
    nombre: String,
    #[max_len(10)]
    libros: Vec<>, //los guaradamos en un arreglo
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, InitSpace, Debug)]
//Estructura de libros
pub struct Libro {
    #[max_len(60)]
    nombre: String,
    paginas: u16, // 2^16, no es necesidad de asignarle espacio
    disponible: bool, //no es necesario, solo recibe t o f
}

//account -> podra interactuar con cuentas en solana
#[derive(Accounts)]
//CREACION DE CONTEXTOS
pub struct NuevaBiblioteca {
    #[account(mut)]
    pub dueno: Signer<'info>, //indica que solo vive cuando sea utilizado
    #[account(
        init,
        payer = dueno,
        space = Biblioteca::INIT_SPACE + 8,
        seeds = [b"biblioteca", dueno.key().as_ref()],
        bump 
        )]
    pub biblioteca: Account<'info, Biblioteca>,
    pub system_program: Program<'info, System>, //siempre va
}

pub struct NuevoLibro {
    pub dueno: Signer<'info>,

    #[account(mut)]
    pub biblioteca: Account<'info, Biblioteca>,
}
