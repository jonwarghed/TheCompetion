import System.Random

data Shot = Hit | Miss deriving Eq

fire :: (Double,Double) -> Shot
fire (a,b) 
    | (a*a + b*b) < 1 = Hit
    | otherwise = Miss

main = do
    gen <- newStdGen
    let infinity = zip (randoms gen :: [Double]) (randoms gen :: [Double])
    let max = 1000000 ::Int
    let hits = length $ filter (==Hit) $ take max $ map fire infinity
    print ((fromIntegral hits) / fromIntegral max * 4)