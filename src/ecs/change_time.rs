use log::warn;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ChangeTime(pub f32); // Utilisation de f32 pour les secondes

impl ChangeTime {
    pub fn new() -> Self {
        Self(0.0) // Démarre à 0 seconde
    }

    pub fn increment(&mut self, delta: f32) {
        self.0 += delta; // Incrémente le temps avec le delta
    }

    // Vérifie si le temps s'est écoulé entre `since` et `current`
    pub fn is_changed_since(self, since: ChangeTime) -> bool {
        let delta = self.0 - since.0;
        if delta.is_infinite() || delta.is_nan() {
            return false; 
        }
        delta > 0.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ComponentTimes {
    pub added: ChangeTime,
    pub changed: ChangeTime,
}

impl ComponentTimes {
    pub fn new(current: ChangeTime) -> Self {
        Self {
            added: current,
            changed: current,
        }
    }

    pub fn set_changed(&mut self, current: ChangeTime) {
        self.changed = current;
    }

    pub fn is_changed_since(&self, since: ChangeTime) -> bool {
        self.changed.is_changed_since(since)
    }

    pub fn is_added_since(&self, since: ChangeTime) -> bool {
        self.added.is_changed_since(since)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_increment() {
        let mut time = ChangeTime::new();
        assert_eq!(time.0, 0.0);
        time.increment(1.0); 
        assert_eq!(time.0, 1.0);
    }

    #[test]
    fn time_is_changed_since_basic() {
        let old = ChangeTime(5.0);
        let current = ChangeTime(10.0);

        // Vérification de changements relatifs
        assert!(current.is_changed_since(old)); // current > old
        assert!(current.is_changed_since(ChangeTime(6.0))); // current > 6.0
        assert!(!old.is_changed_since(current)); // old < current
    }

    #[test]
    fn time_equal_cases() {
        let t = ChangeTime(10.0);
        // Comparaison avec soi-même (aucun changement)
        assert!(!t.is_changed_since(t)); // t n'a pas changé depuis t
    }

    #[test]
    fn component_times() {
        let current = ChangeTime(100.0);
        let mut comp = ComponentTimes::new(current);
        
        // Test de `is_changed_since` pour `ComponentTimes`
        assert!(!comp.is_changed_since(ChangeTime(101.0))); // added = 100.0, since = 101.0, donc aucun changement

        // Mise à jour du temps de changement
        comp.set_changed(ChangeTime(105.0));

        // Test de changement dans le temps
        assert!(comp.is_changed_since(ChangeTime(101.0))); // changed = 105.0, since = 101.0, donc changé
        assert!(!comp.is_changed_since(ChangeTime(110.0))); // changed = 105.0, since = 110.0, donc pas changé

        // Test de `is_added_since` pour `ComponentTimes`
        assert!(comp.is_added_since(ChangeTime(50.0))); // added = 100.0, since = 50.0, donc changé
        assert!(!comp.is_added_since(ChangeTime(110.0))); // added = 100.0, since = 110.0, donc pas ajouté
    }
}
