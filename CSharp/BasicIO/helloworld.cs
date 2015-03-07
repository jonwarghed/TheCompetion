using System;
 
public class HelloWorld
{
    static public void Main ()
    {
        Console.WriteLine ("What is your name: ");
        var name = Console.ReadLine();
        Console.WriteLine(String.format("Hello {0}!",name));
    }
}