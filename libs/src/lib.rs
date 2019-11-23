
#[derive(PartialEq, Debug, Clone)]
pub struct USD (i32);
#[derive(PartialEq, Debug, Clone)]
pub struct GBP (i32);
#[derive(PartialEq, Debug, Clone)]
pub struct CAD (i32);

pub trait ToUSDv<F>{
    fn to_uv(&self,F)->f32;
}

pub trait FromUSDv<F>{
    fn from_uv(&self,f32)->F;
}

impl Account for Ex{
    fn id(&self) -> {self.ac_id}
}

pub struct Ex {
    ac_id:i32,
    cad:f32,
    gbp:f32,
}

pub struct Transaction<A>{
    from_id: i32,
    to_id: i32,
    amount: A,
}

impl ToUSDv<GBP> for Ex {
    fn to_uv(&self, g:GBP)->f32{
        (g.0 as f32) * self.gbp 
    }
}

impl FromUSDv<CAD> for Ex{
    fn from_uv(&self,f:f32)->CAD{
        CAD((f /self.cad) as i32)
    }
}

pub trait Account {
    fn id()->i32;
}

pub trait Exchange<F,T>{
    fn convert(&self, F)->T;
}

impl<E,F,T> Exchange<F,T> for E 
    where E:ToUSDv<F> + FromUSDv<T>
{
    fn convert(&self,f:F)->T {
        self.from_uv(self.to_uv(f)) 
    }
}

pub trait ExchangeAccount<F, T>{
    fn exchange(&self, f_id:i32, t_id:i32, amount) -> 
    (Transaction<F>, Transaction<T>);
}

impl <E,F,T> ExchangeAccount <F, T> for E 
    where E:Exchange<F, T> + Account, F:Clone,
{
    fn exchange(&self, f_id:i32, t_id:i32, amount:F) 
        ->(Transaction<F>, Transaction<T>) {
            let ft = Transaction{from_id:f_id, to_id:self.id(), amount:amount.clone()};
            let tt = Transaction{from_id:self.id(), to_id:t_id, amount:self.convert(amount)};
            (ft, tt)
        }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        // let g = GBP(200);
        // let ex = Ex{cad:0.7,gbp:1.3};
        // let c:CAD = ex.convert(g);

        // assert_eq!(c, CAD(371));

        let ex = Exchange{ac_id:30, cad:0.7, gbp:1.3};

    }
}
