using System;
using System.Random;
using System.Enumerable;

public class Approxpi
{
    public static void Main()
    {
        var gun = new Gun();
        int max = 1000000;
        var shots = Enumerable.Range(1,max).select(x => gun.Fire());
        var hits = shots.count(shot => shot.hit);
        system.println(hits / max * 4);
    }
}

public class Gun
{
    public static random = new Random();

    public Shot Fire()
    {
        a = random.nextDouble();
        b = random.nextDouble();
        return new Shot(){hit = a*a + b*b < 1 }
    }

}

private class Shot
{
    public Boolean hit {get;set;}
}
