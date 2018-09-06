

trait reproductorMP3 {

    fn encender(&self);
    fn apagar(&self);
    fn reproducir(&self);
    fn detener(&self);

}

struct Zune;

impl reproductorMP3 for Zune {

    fn encender(&self) {
        println!("Bienvenido a su iPod.", );
    }

    fn apagar(&self) {
        println!("Gracias por utilizarnos.", );
    }

    fn reproducir(&self) {
        println!("Reproduciendo MegaMix Twenty One Pilots.", );
    }

    fn detener (&self) {
        println!("La musica se ha detenido.", );
    }

}

trait iPod {

    fn on(&self);
    fn off(&self);
    fn play(&self);
    fn stop(&self);

}

struct iPodShuffle;

impl iPod for iPodShuffle {

    fn on(&self) {
        println!("Welcome to your iPodShuffle. Enjoy!", );
    }

    fn off(&self) {
        println!("Good bye!", );
    }

    fn play(&self) {
        println!("Playing ♫ Polarize - Twenty One Pilots ♫", );
    }

    fn stop(&self) {
        println!("Your music is waiting for you, press play to enjoy again.", );
    }

}

struct mp3Adapter {
    mp3 : iPodShuffle
}

impl reproductorMP3 for mp3Adapter {

    fn encender(&self) {
        self.mp3.on();
    }

    fn apagar(&self) {
        self.mp3.off();
    }

    fn reproducir(&self) {
        self.mp3.play();
    }

    fn detener(&self) {
        self.mp3.stop();
    }
}

fn user<S: reproductorMP3>(mp3: &S) {

    mp3.encender();
    mp3.apagar();
    mp3.reproducir();
    mp3.detener();
    println!("\n");

}

fn main() {

    let Gigabeat = Zune;

    println!("Utilizando el reproductor Gigabeat.", );

    user(&Gigabeat);

    let GoGear = iPodShuffle;

    let GoGearAdapter = mp3Adapter {
        mp3: GoGear
    };

    println!("Utilizando el GoGear a traves del mp3mp3Adapter.", );

    user(&GoGearAdapter);
}
