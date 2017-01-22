mod quality_metric;

pub use self::quality_metric::{QualityMetric, UselessQM, TimeQM};

/*
две QM: UselessQM (по умолчанию) и UsefullQM

методы:

QM.switch_on("name");
QM.switch_back();
QM.start("name");
QM.stop();
QM.save(obj);

//возможная проблема: можно сохранять, но нельзя читать

внутри QM:
стэк имен абстрактных функций //?
IB (Information Block) верхнего уровня

структура IB (дерево):
имя
время
память
вектор сериализованных объектов
вектор дочерних IB

QMR = quality metric result

Для каждой AF реализуем метод convert_ib_to_qmr();
и структуру QmrSmth реализующую треит QMR

QMR наследуется от Ord


внутри RAF (Realization Of Abstract Function)
в нужных местах вызываем методы QM
выполняем измерения и сохраняем информацию

По умолчанию в RAF используется UselessQM которая ничего не делает и вычищается компилятором

Если одна AF использует другую, то она также использует методы convert_ib_to_qmr
Следовательно этим можно воспользоваться


Таким образом можно будет сравнивать разные реализации одной и тойже абстрактной функции

*/
