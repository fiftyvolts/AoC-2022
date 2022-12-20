#!/usr/bin/perl

local $" = ",";
my $i    = 0;
my @nums = map { chomp $_; [ $i++, $_ ] } (<ARGV>);
print "0: ", join( ',', map { sprintf "%3d", $_->[1] } @nums ), "\n"
  if $ENV{DEBUG};

my $next = 0;
my $idx  = 0;

while ( $next < @nums ) {

    # print "$idx $next\n";
    my $t = $nums[$idx];

    if ( $t->[0] != $next ) {
        $idx++;
        next;
    }

    my $nidx;
    if ( $t->[1] > 0 ) {
        $nidx = ( $idx + $t->[1] ) % @nums;
    }
    elsif ( $t->[1] < 0 ) {
        $nidx = ( $idx + $t->[1] - 1 ) % @nums;
    }
    else {
        $idx++;
        $next++;
        next;
    }

    if ( $nidx < $idx ) {
        @nums = (
            @nums[ 0 .. $nidx ],
            $t,
            @nums[ $nidx + 1 .. $idx - 1 ],
            @nums[ $idx + 1 .. $#nums ]
        );
    }
    elsif ( $nidx > $idx ) {
        @nums = (
            @nums[ 0 .. $idx - 1 ],
            @nums[ $idx + 1 .. $nidx ],
            $t, @nums[ $nidx + 1 .. $#nums ]
        );
    }
    $next++;
    $idx = 0;
    print "$next: ", join( ',', map { sprintf "%3d", $_->[1] } @nums ), "\n"
      if $ENV{DEBUG};
}

die "didn't find it all" if $next != @nums;

print "0: ", join( ',', map { sprintf "%3d", $_->[1] } @nums ), "\n"
  if $ENV{DEBUG};
my $zero;
for my $idx ( 0 .. $#nums ) {
    if ( $nums[$idx]->[1] == 0 ) {
        $zero = $idx;
    }
}
die "couldn't find zero" if $zero >= @nums;
print "zero at $zero\n";
@final_idx = map { ( $_ + $zero ) % @nums } ( 1000, 2000, 3000 );
@output    = map { $nums[$_]->[1] } @final_idx;
print "@output ", $output[0] + $output[1] + $output[2], "\n";
