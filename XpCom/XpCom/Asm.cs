using System.Reflection;

namespace XpCom
{
    public static class Asm
    {
        public static IEnumerable<Type> Types(this Assembly a) 
            => a.GetTypes().Where(type => type.IsInterface);
    }
}
