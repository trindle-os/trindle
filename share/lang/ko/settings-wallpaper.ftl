# machine translated (Korean), not yet reviewed by a person.

# Settings page "Wallpaper": a plain colour, a picture, or a folder of pictures that change by themselves.

## Page (src/settings/panels/wallpaper.rs)

# Name of the Settings page about the wallpaper, in the list and as its heading.
settings-wallpaper-title = 배경화면
# Words a person might type into the Settings search to find the wallpaper page; separate them with spaces.
settings-wallpaper-keywords = 배경화면 배경 그림 사진 슬라이드쇼 background picture slideshow photo
# Introduction under the heading of the wallpaper page, and the words behind the page i.
settings-wallpaper-help = 단색, 사진 한 장, 또는 원하는 간격으로 바뀌는 사진 폴더를 사용하세요.

## Lists

# Choice in the list of how long each picture stays.
settings-wallpaper-every-10s = 10초
# Choice in the list of how long each picture stays.
settings-wallpaper-every-30s = 30초
# Choice in the list of how long each picture stays.
settings-wallpaper-every-1m = 1분
# Choice in the list of how long each picture stays.
settings-wallpaper-every-5m = 5분
# Choice in the list of how long each picture stays.
settings-wallpaper-every-15m = 15분
# Choice in the list of how long each picture stays.
settings-wallpaper-every-30m = 30분
# Choice in the list of how long each picture stays.
settings-wallpaper-every-1h = 1시간
# Choice in the list of how long each picture stays.
settings-wallpaper-every-1d = 1일
# Choice in the list of how the picture fits the screen.
settings-wallpaper-fit-fill = 화면 채우기
# Choice in the list of how the picture fits the screen.
settings-wallpaper-fit-fit = 사진 전체 맞추기
# Choice in the list of how the picture fits the screen.
settings-wallpaper-fit-center = 가운데, 원래 크기
# Choice in the list of how the picture fits the screen.
settings-wallpaper-fit-stretch = 늘이기
# Choice in the list of how the picture fits the screen.
settings-wallpaper-fit-tile = 바둑판식
# Choice in the list of the order pictures change in: at random.
settings-wallpaper-order-shuffle = 무작위
# Choice in the list of the order pictures change in: by file name.
settings-wallpaper-order-name = 파일 이름순
# Red words when the program that draws the wallpaper would not start.
settings-wallpaper-no-changer = Trindle이 배경화면을 바꾸는 프로그램을 시작하지 못했어요.

## Sentences about the pictures

# Under the large picture, when counting; { $used } of { $all } pictures are used.
settings-wallpaper-count =
    { $all ->
       *[other] 사진 { $all }장 중 { $used }장
    }
# Under the large picture; { $folder } is a folder with no pictures in it.
settings-wallpaper-empty-folder = { $folder }에 사진이 없어요
# Under the large picture, for the pictures that come with Trindle; { $used } of { $all } are used.
settings-wallpaper-count-own =
    { $all ->
       *[other] Trindle과 함께 오는 사진 { $all }장 중 { $used }장
    }
# Under the large picture, for a folder; { $used } of { $all } pictures are used and { $folder } is the folder.
settings-wallpaper-count-folder =
    { $all ->
       *[other] { $folder }의 사진 { $all }장 중 { $used }장
    }
# Under the large picture, for one of Trindle's own pictures; { $name } is its name.
settings-wallpaper-file-own = { $name }, Trindle과 함께 오는 사진 중 하나
# Under the large picture when there is only a colour.
settings-wallpaper-plain-below = 아래에서 정한 단색이에요.
# Under the large picture when the picture or folder has gone.
settings-wallpaper-gone-below = 사진이나 폴더가 더 이상 없어요. 다른 것을 고르세요.
# Sentence about the wallpaper when it is a plain colour.
settings-wallpaper-plain = 단색만 사용해요.
# Sentence about the wallpaper when Trindle's own pictures change; { $every } is how often, { $used } of { $all } pictures are used and { $extra } is any added sentences, each after a space.
settings-wallpaper-changing-own =
    { $all ->
       *[other] Trindle과 함께 오는 사진 { $all }장 중 { $used }장이 { $every }마다 바뀌어요.{ $extra }
    }
# Sentence about the wallpaper when a folder's pictures change; { $every } is how often, { $used } of { $all } pictures are used, { $folder } is the folder's name and { $extra } is any added sentences, each after a space.
settings-wallpaper-changing-folder =
    { $all ->
       *[other] { $folder }의 사진 { $all }장 중 { $used }장이 { $every }마다 바뀌어요.{ $extra }
    }
# Sentence about the wallpaper when its folder has no pictures; { $folder } is the folder.
settings-wallpaper-empty-folder-choose = { $folder } 폴더에 사진이 없어요. 다른 폴더를 고르세요.
# Added to the sentence about the wallpaper when a folder holds more than Trindle looks through.
settings-wallpaper-more-than-looked-at = 폴더에 Trindle이 확인할 수 있는 것보다 많은 사진이 있어서 일부는 빠질 수 있어요.
# Sentence about the wallpaper for one of Trindle's own pictures; { $name } is its name.
settings-wallpaper-file-own-said = { $name }, Trindle과 함께 오는 사진 중 하나예요.
# Sentence about the wallpaper for one chosen picture; { $name } is its name, written to read in the middle of a sentence.
settings-wallpaper-file-one = { $name } 사진을 항상 보여 줘요.
# Sentence about the wallpaper when the chosen picture cannot be used; { $why } is the reason.
settings-wallpaper-file-refused = Trindle이 그 사진을 사용하지 않아요. { $why } 대신 Trindle 기본 사진을 보여 줘요.
# Sentence about the wallpaper when the picture has gone.
settings-wallpaper-file-gone = 사진이 더 이상 없어요. 사진이나 폴더를 고르세요.

## Lines and buttons

# Said in green under a line after its choice was kept.
settings-wallpaper-saved = 저장됨
# Said in green under the next picture line after the picture changed.
settings-wallpaper-changed = 바뀜
# Small word drawn in the corner of a moving picture's tile.
settings-wallpaper-animated = 움직이는 사진
# Title of the window for choosing a picture.
settings-wallpaper-chooser-title = 사진 고르기
# Name of the filter in that window that shows only pictures.
settings-wallpaper-chooser-filter = 사진
# Title of the window for choosing a folder of pictures.
settings-wallpaper-chooser-folder = 사진 폴더 고르기
# Spoken description of the large picture when the wallpaper is a plain colour.
settings-wallpaper-plain-name = 단색
# Name of the large picture of what is on the screen.
settings-wallpaper-on-screen = 화면에 표시 중
# Title of the Wallpaper card.
settings-wallpaper-card-title = 배경화면
# Heading of the wallpaper line, and name of its switch.
settings-wallpaper-wallpaper = 배경화면
# Heading of the line for choosing where the picture comes from.
settings-wallpaper-picture = 사진
# Name of the round button that chooses one picture.
settings-wallpaper-choose-picture-button = 사진 고르기
# Name of the round button that chooses a folder of pictures.
settings-wallpaper-choose-folder = 폴더 고르기
# Name of the round button that chooses Trindle's own pictures.
settings-wallpaper-own-pictures = Trindle 기본 사진
# Heading of the fit line.
settings-wallpaper-fit = 맞춤
# Name of the list of how the picture fits the screen.
settings-wallpaper-fit-name = 화면에 맞추는 방법
# Heading of the colour line.
settings-wallpaper-colour = 색
# Small words under the colour line when a picture covers it.
settings-wallpaper-colour-hidden = 사진에 가려져 있어요.
# Placeholder and name of the box for typing the colour as a hex code; the example is a code.
settings-wallpaper-colour-hex = #09090b 같은 16진수 코드
# Words behind the i of the Wallpaper card.
settings-wallpaper-card-about = 단색, 사진 한 장, 또는 원하는 간격으로 바뀌는 사진 폴더를 사용하세요. 사진이 없을 때와 사진이 화면을 다 덮지 않을 때 가장자리에 이 색이 보여요. #09090b 같은 16진수 코드로 입력하세요.
# Small words on the cards that are greyed while the wallpaper is off.
settings-wallpaper-waits = 사용하려면 배경화면을 켜세요.
# Heading of the line, and name of its switch, for changing the picture now and then.
settings-wallpaper-rotate = 가끔 사진 바꾸기
# Heading of the line, and name of its list, for how long each picture stays.
settings-wallpaper-every = 바꾸는 간격
# Heading of the line, and name of its list, for the order pictures change in.
settings-wallpaper-order = 순서
# Heading of the line, and name of its button, for moving to the next picture.
settings-wallpaper-next = 다음 사진
# Title of the Changing pictures card.
settings-wallpaper-changing-title = 바뀌는 사진
# Words behind the i of that card.
settings-wallpaper-changing-about = 폴더의 모든 사진을 여기서 정한 간격으로 차례로 보여 줘요. 무작위 또는 파일 이름순으로 고를 수 있어요.
# Words behind the i of the Pictures card when pictures change by themselves.
settings-wallpaper-pictures-rotating = Trindle은 흐리지 않은 사진을 모두 사용해요. 사진을 누르면 건너뛰고, 다시 누르면 돌아와요. 삭제되는 것은 없어요. 큰 사진이 지금 화면에 있는 사진이에요.
# Words behind the i of the Pictures card when one picture is on.
settings-wallpaper-pictures-one = 사진을 눌러 사용하세요.
# Title of the Pictures card.
settings-wallpaper-pictures-title = 사진
# Spoken description of the empty frame shown when there are no pictures.
settings-wallpaper-no-pictures-name = 아직 사진이 없어요
# Heading of the line shown when there are no pictures to choose from.
settings-wallpaper-no-pictures = 고를 사진이 없어요
