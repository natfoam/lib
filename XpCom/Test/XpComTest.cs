using System.Reflection;
using XpCom;

namespace Test
{
    [TestClass]
    public class XpComTest
    {
        [TestMethod]
        public void ExampleTest()
        {
            var types = Assembly.GetAssembly(typeof(Example.IMy))!.Types().ToArray();
           CollectionAssert.AreEqual(types, new[] { typeof(Example.IMy) });
        }
    }
}