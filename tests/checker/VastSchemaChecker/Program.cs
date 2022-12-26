using System.Xml.Linq;
using System.Xml.Schema;

Console.WriteLine("Start VAST schema checker!");

XNamespace ns = "http://www.iab.com/VAST";

var schema = new XmlSchemaSet();
schema.Add(ns.ToString(), "./vast_4.2.xsd");

foreach (var file in Directory.GetFiles("../../output/v4_2"))
{
    Console.WriteLine($"Check '{file}'");
    var xdoc = XDocument.Load("../../output/v4_2/Ad_Verification-test.xml");
    xdoc.Root!.Name = ns + xdoc.Root!.Name.LocalName;
    // Console.WriteLine($"Input XDocument: {xdoc}");

    xdoc.Validate(schema, static (object? sender, ValidationEventArgs e) =>
    {
        // Console.WriteLine($"sender: {sender}");
        // Console.WriteLine($"args: {e}");
        Console.WriteLine($"Message: {e.Message}");
    });
}

Console.WriteLine("Done!");
