using System;
using System.Linq;

public class Approxpi
{
    public static void Main()
    {
        var gun = new Gun();
        int max = 1000000;
        var shots = Enumerable.Range(1,max).Select(x => gun.Fire());
        var hits = shots.Count(shot => shot.hit);
        Console.WriteLine((double)hits / max * 4);
    }
}

public class Gun
{
    public Gun()
    {
        random = new Random();
    }
    private Random random;

    public Shot Fire()
    {
        var a = random.NextDouble();
        var b = random.NextDouble();
        return new Shot(){hit = a*a + b*b < 1 };
    }

}

public class Shot
{
    public Boolean hit {get;set;}
}
