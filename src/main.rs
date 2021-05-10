//型パラメータT
struct Point<T>{
    x: T,
    y: T,
}

//ジェネリックなデータ型のimplブロック
impl<T> Point<T>{
    fn new(x: T, y: T) -> Point<T> {
        Point{x, y}
    }

    fn x(&self) -> &T{
        &self.x
    }
}

/* トレイト＝構造体同士がもつ共通の機能 */
trait Sensor{
    fn read(&self) -> u32;

    //デフォルト実装
    //各構造体のimplで新たに書かない限りはこれが実行される
    fn fill(&self, buffer: &mut [u32]){
        for element in buffer.iter_mut(){
            *element = self.read();
        }
    }
}

//構造体へのトレイトの実装
struct LightSensor{
    value: u32,
}

impl Sensor for LightSensor{
    fn read(&self) -> u32{
        self.value
    }
}

struct TemperatureSensor{
    value: f32,
}

impl Sensor for TemperatureSensor{
    fn read(&self) -> u32{
        self.value as u32
    }
}

fn main() {
    //::<:i32>のように型パラメータを指定する
    let p_i32 = Point::<i32>{x: 300, y: 200};
    let p_i8  = Point::<i8>{x: 10, y: 20};

    use std::mem::size_of_val;
    println!("size of x in p_i32 = {}", size_of_val(&p_i32.x));
    println!("size of x in p_i8 = {}", size_of_val(&p_i8.x));

    /* これらはすべて同義（コンパイラが型推論可能であればよい） */
    /*
    let p_i32 = Point::<i32>{x: 300, y: 200};
    let p_i32: Point<i32> = Point{x: 300, y: 200};
    let p_i32 = Point{x: 300i32, y: 200i32};
    let p_i32 = Point{x: 300, y: 200}; //整数リテラルはデフォルトでi32
    */

    /* ジェネリック関数 */
    //ジェネリック関数の呼び出し
    let p_i32 = make_point::<i32>(300, 200);
    let p_i8  = make_point::<i8>(10, 20);

    //こちらも型推論可能な場合は省略
    let p = make_point(1, 2);

    /* トレイトにて実装されたメソッドの呼び出し */
    let light_sensor = LightSensor{value: 42};
    let temperature_sensor = TemperatureSensor{value: 32.1};

    println!("light sensor value  {}", light_sensor.read());
    println!("temperature sensor value {}", temperature_sensor.read());

    print_sensor_value(light_sensor);
    print_sensor_value(temperature_sensor);
    //i32はSensorトレイトを実装していないのでエラー
    //print_sensor_value(42i32);
}

//ジェネリック関数
fn make_point<T>(x: T, y:T) -> Point<T>{
    Point{x, y}
}

/* 関数パラメーターとしてトレイトを使用 */
fn print_sensor_value(sensor: impl Sensor){
    //sensorにはSenSorトレイトが実装されたオブジェクトのみ許容
    println!("sensor value = {}", sensor.read());
}

/* impl Sensorを用いたコードと等価 */
fn print_sensor_value_b<S: Sensor>(sensor: S){
    //S: SensorにおいてSが型パラメータ，Sensorがトレイト境界
    //トレイト境界は型パラメータに制約を追加する
    println!("sensor value = {}", sensor.read());
}

//whereを使ったトレイト境界
fn print_sensor_value_w<S>(sensor: S)
    where S: Sensor,
{
    //S: SensorにおいてSが型パラメータ，Sensorがトレイト境界
    //トレイト境界は型パラメータに制約を追加する
    println!("sensor value = {}", sensor.read());

}

// Sensor + Debug　のように複数のトレイトを追加可能
//ジェネリクスはコンパイル時に解決されるため，実行時に一切のオーバーヘッドがかからない
//→ゼロコスト抽象化
