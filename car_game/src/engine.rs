struct Engine {
    displacement: f32,  // Объем двигателя
    cylinders: u8,  // Кол-во цилиндров
    max_power: f32, // Максимальная мощность
    max_troque: f32,    // Максимальный крутящий момент
    max_rpm: u32,   // Максимально допустимые обороты
    idle_rpm: u32,  // обороты холостого хода
    intertia: f32,  // Инерция маховика и коленвала
}

impl Engine {
    // Просто возвращаем значение поля
    pub fn get_displacement(&self) -> f32 {
        self.displacement
    }
    
    pub fn get_cylinders(&self) -> u8 {
        self.cylinders
    }
    
    pub fn get_max_rpm(&self) -> u32 {
        self.max_rpm
    }
    
    pub fn get_idle_rpm(&self) -> u32 {
        self.idle_rpm
    }
    
    pub fn get_inertia(&self) -> f32 {
        self.inertia
    }
}
